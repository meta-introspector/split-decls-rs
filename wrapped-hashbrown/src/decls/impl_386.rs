macro_rules! deps {
    () => {
        ScopeGuard!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl < T , F > Drop for ScopeGuard < T , F > where F : FnMut (& mut T) , { # [inline] fn drop (& mut self) { (self . dropfn) (& mut self . value) ; } }
    };
}

impl_386!()