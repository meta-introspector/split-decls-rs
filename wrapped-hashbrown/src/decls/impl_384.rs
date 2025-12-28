macro_rules! deps {
    () => {
        ScopeGuard!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < T , F > Deref for ScopeGuard < T , F > where F : FnMut (& mut T) , { type Target = T ; # [inline] fn deref (& self) -> & T { & self . value } }
    };
}

impl_384!();