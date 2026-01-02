mkuse!{use core :: arch :: asm ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: os :: xous :: ffi :: { MemoryFlags , map_memory , unmap_memory } ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , AtomicUsize } ;}
mkitem!{pub type Key = usize ;}
mkitem!{pub type Dtor = unsafe extern "C" fn (* mut u8) ;}
mkitem!{const TLS_MEMORY_SIZE : usize = 4096 ;}
mkitem!{# [doc = " TLS keys start at `1`. Index `0` is unused"] # [cfg (not (test))] # [unsafe (export_name = "_ZN16__rust_internals3std3sys4xous16thread_local_key13TLS_KEY_INDEXE")] static TLS_KEY_INDEX : Atomic < usize > = AtomicUsize :: new (1) ;}
mkitem!{# [cfg (not (test))] # [unsafe (export_name = "_ZN16__rust_internals3std3sys4xous16thread_local_key9DTORSE")] static DTORS : Atomic < * mut Node > = AtomicPtr :: new (ptr :: null_mut ()) ;}
mkitem!{# [cfg (test)] unsafe extern "Rust" { # [link_name = "_ZN16__rust_internals3std3sys4xous16thread_local_key13TLS_KEY_INDEXE"] static TLS_KEY_INDEX : Atomic < usize > ; # [link_name = "_ZN16__rust_internals3std3sys4xous16thread_local_key9DTORSE"] static DTORS : Atomic < * mut Node > ; }}

macro_rules! tls_ptr_addr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tls_ptr_addr in module {}", module_path!());
    };
}

mkfn!{
    tls_ptr_addr_introspect!();
    fn tls_ptr_addr () -> * mut * mut u8 { let mut tp : usize ; unsafe { asm ! ("mv {}, tp" , out (reg) tp ,) ; } core :: ptr :: with_exposed_provenance_mut :: < * mut u8 > (tp) }
}

macro_rules! tls_table_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tls_table in module {}", module_path!());
    };
}

mkfn!{
    tls_table_introspect!();
    # [doc = " Creates an area of memory that's unique per thread. This area will"] # [doc = " contain all thread local pointers."] fn tls_table () -> & 'static mut [* mut u8] { let tp = tls_ptr_addr () ; if ! tp . is_null () { return unsafe { core :: slice :: from_raw_parts_mut (tp , TLS_MEMORY_SIZE / size_of :: < * mut u8 > ()) } ; } let tp = unsafe { map_memory (None , None , TLS_MEMORY_SIZE / size_of :: < * mut u8 > () , MemoryFlags :: R | MemoryFlags :: W ,) . expect ("Unable to allocate memory for thread local storage") } ; for val in tp . iter () { assert ! (* val as usize == 0) ; } unsafe { asm ! ("mv tp, {}" , in (reg) tp . as_mut_ptr () as usize ,) ; } tp }
}

macro_rules! create_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create in module {}", module_path!());
    };
}

mkfn!{
    create_introspect!();
    # [inline] pub fn create (dtor : Option < Dtor >) -> Key { # [allow (unused_unsafe)] let key = unsafe { TLS_KEY_INDEX . fetch_add (1 , Relaxed) } ; if let Some (f) = dtor { unsafe { register_dtor (key , f) } ; } key }
}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    # [inline] pub unsafe fn set (key : Key , value : * mut u8) { assert ! ((key < 1022) && (key >= 1)) ; let table = tls_table () ; table [key] = value ; }
}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    # [inline] pub unsafe fn get (key : Key) -> * mut u8 { assert ! ((key < 1022) && (key >= 1)) ; tls_table () [key] }
}

macro_rules! destroy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy in module {}", module_path!());
    };
}

mkfn!{
    destroy_introspect!();
    # [inline] pub unsafe fn destroy (_key : Key) { }
}
mkitem!{mkstruct!{struct Node { dtor : Dtor , key : Key , next : * mut Node , }}}

macro_rules! register_dtor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function register_dtor in module {}", module_path!());
    };
}

mkfn!{
    register_dtor_introspect!();
    unsafe fn register_dtor (key : Key , dtor : Dtor) { let mut node = ManuallyDrop :: new (Box :: new (Node { key , dtor , next : ptr :: null_mut () })) ; # [allow (unused_unsafe)] let mut head = unsafe { DTORS . load (Acquire) } ; loop { node . next = head ; # [allow (unused_unsafe)] match unsafe { DTORS . compare_exchange (head , & mut * * node , Release , Acquire) } { Ok (_) => return , Err (cur) => head = cur , } } }
}

macro_rules! destroy_tls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy_tls in module {}", module_path!());
    };
}

mkfn!{
    destroy_tls_introspect!();
    pub unsafe fn destroy_tls () { let tp = tls_ptr_addr () ; if tp . is_null () { return ; } unsafe { run_dtors () } ; unsafe { unmap_memory (core :: slice :: from_raw_parts_mut (tp , TLS_MEMORY_SIZE / size_of :: < usize > ())) . unwrap () } ; }
}

macro_rules! run_dtors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_dtors in module {}", module_path!());
    };
}

mkfn!{
    run_dtors_introspect!();
    unsafe fn run_dtors () { let mut any_run = true ; for _ in 0 .. 5 { if ! any_run { break ; } any_run = false ; # [allow (unused_unsafe)] let mut cur = unsafe { DTORS . load (Acquire) } ; while ! cur . is_null () { let ptr = unsafe { get ((* cur) . key) } ; if ! ptr . is_null () { unsafe { set ((* cur) . key , ptr :: null_mut ()) } ; unsafe { ((* cur) . dtor) (ptr as * mut _) } ; any_run = true ; } unsafe { cur = (* cur) . next } ; } } crate :: rt :: thread_cleanup () ; }
}