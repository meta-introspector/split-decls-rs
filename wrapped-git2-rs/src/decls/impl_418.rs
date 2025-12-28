macro_rules! deps {
    () => {
        Buf!();
        Mempack!();
        Repository!();
        Error!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl < 'odb > Mempack < 'odb > { # [doc = " Dumps the contents of the mempack into the provided buffer."] pub fn dump (& self , repo : & Repository , buf : & mut Buf) -> Result < () , Error > { unsafe { try_call ! (raw :: git_mempack_dump (buf . raw () , repo . raw () , self . raw)) ; } Ok (()) } # [doc = " Clears all data in the mempack."] pub fn reset (& self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_mempack_reset (self . raw)) ; } Ok (()) } }
    };
}

impl_418!();