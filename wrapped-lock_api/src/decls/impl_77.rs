macro_rules! deps {
    () => {
        ReentrantMutexGuard!();
        RawMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for ReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_77!()