mkuse!{use crate :: sync :: atomic :: Ordering :: Relaxed ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool } ;}
mkitem!{static RNG_INIT : Atomic < bool > = AtomicBool :: new (false) ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (mut bytes : & mut [u8]) { while ! RNG_INIT . load (Relaxed) { let ret = unsafe { libc :: randSecure () } ; if ret < 0 { panic ! ("failed to generate random data") ; } else if ret > 0 { RNG_INIT . store (true , Relaxed) ; break ; } unsafe { libc :: usleep (10) } ; } while ! bytes . is_empty () { let len = bytes . len () . try_into () . unwrap_or (libc :: c_int :: MAX) ; let ret = unsafe { libc :: randABytes (bytes . as_mut_ptr () , len) } ; assert ! (ret >= 0 , "failed to generate random data") ; bytes = & mut bytes [len as usize ..] ; } }
}