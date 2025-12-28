macro_rules! deps {
    () => {
        ShardedLock!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Debug > fmt :: Debug for ShardedLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_read () { Ok (guard) => f . debug_struct ("ShardedLock") . field ("data" , & & * guard) . finish () , Err (TryLockError :: Poisoned (err)) => f . debug_struct ("ShardedLock") . field ("data" , & & * * err . get_ref ()) . finish () , Err (TryLockError :: WouldBlock) => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("ShardedLock") . field ("data" , & LockedPlaceholder) . finish () } } } }
    };
}

impl_121!();