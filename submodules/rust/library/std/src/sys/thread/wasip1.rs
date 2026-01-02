mkuse!{# [cfg (target_feature = "atomics")] use crate :: io ;}
mkuse!{use crate :: mem ;}
mkuse!{# [cfg (target_feature = "atomics")] use crate :: num :: NonZero ;}
mkuse!{# [cfg (target_feature = "atomics")] use crate :: sys :: os ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{# [cfg (target_feature = "atomics")] use crate :: { cmp , ptr } ;}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{pub use libc :: * ;}
mkuse!{pub use crate :: ffi ;}
mkitem!{# [repr (C)] union pthread_attr_union { __i : [ffi :: c_int ; if size_of :: < ffi :: c_long > () == 8 { 14 } else { 9 }] , __vi : [ffi :: c_int ; if size_of :: < ffi :: c_long > () == 8 { 14 } else { 9 }] , __s : [ffi :: c_ulong ; if size_of :: < ffi :: c_long > () == 8 { 7 } else { 9 }] , }}
mkitem!{mkstruct!{# [repr (C)] pub struct pthread_attr_t { __u : pthread_attr_union , }}}
mkitem!{# [allow (non_camel_case_types)] pub type pthread_t = * mut ffi :: c_void ;}
mkitem!{pub const _SC_NPROCESSORS_ONLN : ffi :: c_int = 84 ;}
mkitem!{unsafe extern "C" { pub fn pthread_create (native : * mut pthread_t , attr : * const pthread_attr_t , f : extern "C" fn (* mut ffi :: c_void) -> * mut ffi :: c_void , value : * mut ffi :: c_void ,) -> ffi :: c_int ; pub fn pthread_join (native : pthread_t , value : * mut * mut ffi :: c_void) -> ffi :: c_int ; pub fn pthread_attr_init (attrp : * mut pthread_attr_t) -> ffi :: c_int ; pub fn pthread_attr_setstacksize (attr : * mut pthread_attr_t , stack_size : libc :: size_t ,) -> ffi :: c_int ; pub fn pthread_attr_destroy (attr : * mut pthread_attr_t) -> ffi :: c_int ; pub fn pthread_detach (thread : pthread_t) -> ffi :: c_int ; }} 
            }}
mkitem!{mkstruct!{# [cfg (target_feature = "atomics")] pub struct Thread { id : libc :: pthread_t , }}}
mkitem!{mkimpl!{# [cfg (target_feature = "atomics")] impl Drop for Thread { fn drop (& mut self) { let ret = unsafe { libc :: pthread_detach (self . id) } ; debug_assert_eq ! (ret , 0) ; } }}}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 1024 * 1024 ;}
mkitem!{mkimpl!{# [cfg (target_feature = "atomics")] impl Thread { pub unsafe fn new (stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { let p = Box :: into_raw (Box :: new (p)) ; let mut native : libc :: pthread_t = unsafe { mem :: zeroed () } ; let mut attr : libc :: pthread_attr_t = unsafe { mem :: zeroed () } ; assert_eq ! (unsafe { libc :: pthread_attr_init (& mut attr) } , 0) ; let stack_size = cmp :: max (stack , DEFAULT_MIN_STACK_SIZE) ; match unsafe { libc :: pthread_attr_setstacksize (& mut attr , stack_size) } { 0 => { } n => { assert_eq ! (n , libc :: EINVAL) ; let page_size = os :: page_size () ; let stack_size = (stack_size + page_size - 1) & (- (page_size as isize - 1) as usize - 1) ; assert_eq ! (unsafe { libc :: pthread_attr_setstacksize (& mut attr , stack_size) } , 0) ; } } ; let ret = unsafe { libc :: pthread_create (& mut native , & attr , thread_start , p as * mut _) } ; assert_eq ! (unsafe { libc :: pthread_attr_destroy (& mut attr) } , 0) ; return if ret != 0 { unsafe { drop (Box :: from_raw (p)) ; } Err (io :: Error :: from_raw_os_error (ret)) } else { Ok (Thread { id : native }) } ; extern "C" fn thread_start (main : * mut libc :: c_void) -> * mut libc :: c_void { unsafe { Box :: from_raw (main as * mut Box < dyn FnOnce () >) () ; } ptr :: null_mut () } } pub fn join (self) { let id = mem :: ManuallyDrop :: new (self) . id ; let ret = unsafe { libc :: pthread_join (id , ptr :: null_mut ()) } ; if ret != 0 { rtabort ! ("failed to join thread: {}" , io :: Error :: from_raw_os_error (ret)) ; } } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    # [cfg (target_feature = "atomics")] pub fn available_parallelism () -> io :: Result < NonZero < usize > > { match unsafe { libc :: sysconf (libc :: _SC_NPROCESSORS_ONLN) } { - 1 => Err (io :: Error :: last_os_error ()) , cpus => NonZero :: new (cpus as usize) . ok_or (io :: Error :: UNKNOWN_THREAD_COUNT) , } }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { let ret = unsafe { wasi :: sched_yield () } ; debug_assert_eq ! (ret , Ok (())) ; }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { let mut nanos = dur . as_nanos () ; while nanos > 0 { const USERDATA : wasi :: Userdata = 0x0123_45678 ; let clock = wasi :: SubscriptionClock { id : wasi :: CLOCKID_MONOTONIC , timeout : u64 :: try_from (nanos) . unwrap_or (u64 :: MAX) , precision : 0 , flags : 0 , } ; nanos -= u128 :: from (clock . timeout) ; let in_ = wasi :: Subscription { userdata : USERDATA , u : wasi :: SubscriptionU { tag : 0 , u : wasi :: SubscriptionUU { clock } } , } ; unsafe { let mut event : wasi :: Event = mem :: zeroed () ; let res = wasi :: poll_oneoff (& in_ , & mut event , 1) ; match (res , event) { (Ok (1) , wasi :: Event { userdata : USERDATA , error : wasi :: ERRNO_SUCCESS , type_ : wasi :: EVENTTYPE_CLOCK , .. } ,) => { } _ => panic ! ("thread::sleep(): unexpected result of poll_oneoff") , } } } }
}