macro_rules! deps {
    () => {
        ReentrantMutex!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < R : RawMutex , G : GetThreadId , T : ? Sized + fmt :: Debug > fmt :: Debug for ReentrantMutex < R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => f . debug_struct ("ReentrantMutex") . field ("data" , & & * guard) . finish () , None => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("ReentrantMutex") . field ("data" , & LockedPlaceholder) . finish () } } } }
    };
}

impl_68!();