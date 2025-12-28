macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < R : RawRwLock , T : ? Sized + fmt :: Debug > fmt :: Debug for RwLock < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("RwLock") ; match self . try_read () { Some (guard) => d . field ("data" , & & * guard) , None => { d . field ("data" , & format_args ! ("<locked>")) } } ; d . finish () } }
    };
}

impl_121!()