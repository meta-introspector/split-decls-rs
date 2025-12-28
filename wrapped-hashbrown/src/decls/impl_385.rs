macro_rules! deps {
    () => {
        ScopeGuard!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < T , F > DerefMut for ScopeGuard < T , F > where F : FnMut (& mut T) , { # [inline] fn deref_mut (& mut self) -> & mut T { & mut self . value } }
    };
}

impl_385!();