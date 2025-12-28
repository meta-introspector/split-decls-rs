macro_rules! deps {
    () => {
        RawStderr!();
        Error!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl fmt :: Write for RawStderr { fn write_str (& mut self , s : & str) -> Result < () , fmt :: Error > { let ret = unsafe { libc :: write (libc :: STDERR_FILENO , s . as_ptr () . cast () , s . len ()) } ; if ret == - 1 { Err (fmt :: Error) } else { Ok (()) } } }
    };
}

impl_54!();