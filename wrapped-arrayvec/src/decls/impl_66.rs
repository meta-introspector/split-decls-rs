macro_rules! deps {
    () => {
        ScopeExitGuard!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T , Data , F > Drop for ScopeExitGuard < T , Data , F > where F : FnMut (& Data , & mut T) , { fn drop (& mut self) { (self . f) (& self . data , & mut self . value) } }
    };
}

impl_66!()