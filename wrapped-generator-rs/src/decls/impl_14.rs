macro_rules! deps {
    () => {
        Generator!();
        Scope!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , A , T > Generator < 'a , A , T > { # [doc = " init a heap based generator with scoped closure"] pub fn scoped_init < F > (& mut self , f : F) where for < 'scope > F : FnOnce (Scope < 'scope , 'a , A , T >) -> T + Send + 'a , T : Send + 'a , A : Send + 'a , { self . gen . scoped_init (f) ; } # [doc = " init a heap based generator"] pub fn init_code < F : FnOnce () -> T + Send + 'a > (& mut self , f : F) where T : Send + 'a , { self . gen . init_code (f) ; } }
    };
}

impl_14!();