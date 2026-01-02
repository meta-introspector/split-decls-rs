mkuse!{use crate :: fs :: File ;}
mkuse!{use crate :: io :: Read ;}
mkuse!{use crate :: os :: fd :: AsRawFd ;}
mkuse!{use crate :: sync :: OnceLock ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool } ;}
mkuse!{use crate :: sys :: pal :: os :: errno ;}
mkuse!{use crate :: sys :: pal :: weak :: syscall ;}

macro_rules! getrandom_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getrandom in module {}", module_path!());
    };
}

mkfn!{
    getrandom_introspect!();
    fn getrandom (mut bytes : & mut [u8] , insecure : bool) { syscall ! (fn getrandom (buffer : * mut libc :: c_void , length : libc :: size_t , flags : libc :: c_uint ,) -> libc :: ssize_t ;) ; static GETRANDOM_AVAILABLE : Atomic < bool > = AtomicBool :: new (true) ; static GRND_INSECURE_AVAILABLE : Atomic < bool > = AtomicBool :: new (true) ; static URANDOM_READY : Atomic < bool > = AtomicBool :: new (false) ; static DEVICE : OnceLock < File > = OnceLock :: new () ; if GETRANDOM_AVAILABLE . load (Relaxed) { loop { if bytes . is_empty () { return ; } let flags = if insecure { if GRND_INSECURE_AVAILABLE . load (Relaxed) { libc :: GRND_INSECURE } else { libc :: GRND_NONBLOCK } } else { 0 } ; let ret = unsafe { getrandom (bytes . as_mut_ptr () . cast () , bytes . len () , flags) } ; if ret != - 1 { bytes = & mut bytes [ret as usize ..] ; } else { match errno () { libc :: EINTR => continue , libc :: EINVAL if flags == libc :: GRND_INSECURE => { GRND_INSECURE_AVAILABLE . store (false , Relaxed) ; continue ; } libc :: EAGAIN if flags == libc :: GRND_NONBLOCK => break , libc :: ENOSYS | libc :: EPERM => { GETRANDOM_AVAILABLE . store (false , Relaxed) ; break ; } _ => panic ! ("failed to generate random data") , } } } } if ! insecure { if ! URANDOM_READY . load (Acquire) { let random = File :: open ("/dev/random") . expect ("failed to open /dev/random") ; let mut fd = libc :: pollfd { fd : random . as_raw_fd () , events : libc :: POLLIN , revents : 0 } ; while ! URANDOM_READY . load (Acquire) { let ret = unsafe { libc :: poll (& mut fd , 1 , - 1) } ; match ret { 1 => { assert_eq ! (fd . revents , libc :: POLLIN) ; URANDOM_READY . store (true , Release) ; break ; } - 1 if errno () == libc :: EINTR => continue , _ => panic ! ("poll(\"/dev/random\") failed") , } } } } DEVICE . get_or_try_init (| | File :: open ("/dev/urandom")) . and_then (| mut dev | dev . read_exact (bytes)) . expect ("failed to generate random data") ; }
}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { getrandom (bytes , false) ; }
}

macro_rules! hashmap_random_keys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hashmap_random_keys in module {}", module_path!());
    };
}

mkfn!{
    hashmap_random_keys_introspect!();
    pub fn hashmap_random_keys () -> (u64 , u64) { let mut bytes = [0 ; 16] ; getrandom (& mut bytes , true) ; let k1 = u64 :: from_ne_bytes (bytes [.. 8] . try_into () . unwrap ()) ; let k2 = u64 :: from_ne_bytes (bytes [8 ..] . try_into () . unwrap ()) ; (k1 , k2) }
}