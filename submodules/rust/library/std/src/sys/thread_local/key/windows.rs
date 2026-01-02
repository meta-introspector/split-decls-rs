mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { AcqRel , Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , AtomicU32 } ;}
mkuse!{use crate :: sys :: c ;}
mkuse!{use crate :: sys :: thread_local :: guard ;}
mkitem!{pub type Key = u32 ;}
mkitem!{type Dtor = unsafe extern "C" fn (* mut u8) ;}
mkitem!{mkstruct!{pub struct LazyKey { # [doc = " The key value shifted up by one. Since TLS_OUT_OF_INDEXES == u32::MAX"] # [doc = " is not a valid key value, this allows us to use zero as sentinel value"] # [doc = " without risking overflow."] key : Atomic < Key > , dtor : Option < Dtor > , next : Atomic < * mut LazyKey > , # [doc = " Currently, destructors cannot be unregistered, so we cannot use racy"] # [doc = " initialization for keys. Instead, we need synchronize initialization."] # [doc = " Use the Windows-provided `Once` since it does not require TLS."] once : UnsafeCell < c :: INIT_ONCE > , }}}
mkitem!{mkimpl!{impl LazyKey { # [inline] pub const fn new (dtor : Option < Dtor >) -> LazyKey { LazyKey { key : AtomicU32 :: new (0) , dtor , next : AtomicPtr :: new (ptr :: null_mut ()) , once : UnsafeCell :: new (c :: INIT_ONCE_STATIC_INIT) , } } # [inline] pub fn force (& 'static self) -> Key { match self . key . load (Acquire) { 0 => unsafe { self . init () } , key => key - 1 , } } # [cold] unsafe fn init (& 'static self) -> Key { if self . dtor . is_some () { let mut pending = c :: FALSE ; let r = unsafe { c :: InitOnceBeginInitialize (self . once . get () , 0 , & mut pending , ptr :: null_mut ()) } ; assert_eq ! (r , c :: TRUE) ; if pending == c :: FALSE { self . key . load (Relaxed) - 1 } else { let key = unsafe { c :: TlsAlloc () } ; if key == c :: TLS_OUT_OF_INDEXES { rtabort ! ("out of TLS indexes") ; } unsafe { register_dtor (self) ; } self . key . store (key + 1 , Release) ; let r = unsafe { c :: InitOnceComplete (self . once . get () , 0 , ptr :: null_mut ()) } ; debug_assert_eq ! (r , c :: TRUE) ; key } } else { let key = unsafe { c :: TlsAlloc () } ; if key == c :: TLS_OUT_OF_INDEXES { rtabort ! ("out of TLS indexes") ; } match self . key . compare_exchange (0 , key + 1 , AcqRel , Acquire) { Ok (_) => key , Err (new) => unsafe { let r = c :: TlsFree (key) ; debug_assert_eq ! (r , c :: TRUE) ; new - 1 } , } } } }}}
mkitem!{mkimpl!{unsafe impl Send for LazyKey { }}}
mkitem!{mkimpl!{unsafe impl Sync for LazyKey { }}}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    # [inline] pub unsafe fn set (key : Key , val : * mut u8) { let r = unsafe { c :: TlsSetValue (key , val . cast ()) } ; debug_assert_eq ! (r , c :: TRUE) ; }
}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    # [inline] pub unsafe fn get (key : Key) -> * mut u8 { unsafe { c :: TlsGetValue (key) . cast () } }
}
mkitem!{static DTORS : Atomic < * mut LazyKey > = AtomicPtr :: new (ptr :: null_mut ()) ;}

macro_rules! register_dtor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function register_dtor in module {}", module_path!());
    };
}

mkfn!{
    register_dtor_introspect!();
    # [doc = " Should only be called once per key, otherwise loops or breaks may occur in"] # [doc = " the linked list."] unsafe fn register_dtor (key : & 'static LazyKey) { guard :: enable () ; let this = < * const LazyKey > :: cast_mut (key) ; let mut head = DTORS . load (Acquire) ; loop { key . next . store (head , Relaxed) ; match DTORS . compare_exchange_weak (head , this , Release , Acquire) { Ok (_) => break , Err (new) => head = new , } } }
}

macro_rules! run_dtors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_dtors in module {}", module_path!());
    };
}

mkfn!{
    run_dtors_introspect!();
    # [doc = " This will and must only be run by the destructor callback in [`guard`]."] pub unsafe fn run_dtors () { for _ in 0 .. 5 { let mut any_run = false ; let mut cur = DTORS . load (Acquire) ; while ! cur . is_null () { let pre_key = unsafe { (* cur) . key . load (Acquire) } ; let dtor = unsafe { (* cur) . dtor . unwrap () } ; cur = unsafe { (* cur) . next . load (Relaxed) } ; if pre_key == 0 { continue ; } let key = pre_key - 1 ; let ptr = unsafe { c :: TlsGetValue (key) } ; if ! ptr . is_null () { unsafe { c :: TlsSetValue (key , ptr :: null_mut ()) ; dtor (ptr as * mut _) ; any_run = true ; } } } if ! any_run { break ; } } }
}