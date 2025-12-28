macro_rules! deps {
    () => {
        Scope!();
        LocalGenerator!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , A , T > LocalGenerator < 'a , A , T > { # [doc = " init a heap based generator with scoped closure"] pub fn scoped_init < F > (& mut self , f : F) where for < 'scope > F : FnOnce (Scope < 'scope , 'a , A , T >) -> T + 'a , T : 'a , A : 'a , { self . gen . scoped_init (f) ; } }
    };
}

impl_16!()