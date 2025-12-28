macro_rules! deps {
    () => {
        Scope!();
        Generator!();
        GeneratorImpl!();
        Gn!();
        LocalGenerator!();
        Stack!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < A > Gn < A > { # [doc = " create a scoped generator with default stack size"] pub fn new_scoped < 'a , T , F > (f : F) -> Generator < 'a , A , T > where for < 'scope > F : FnOnce (Scope < 'scope , 'a , A , T >) -> T + Send + 'a , T : Send + 'a , A : Send + 'a , { Self :: new_scoped_opt (DEFAULT_STACK_SIZE , f) } # [doc = " create a scoped local generator with default stack size"] pub fn new_scoped_local < 'a , T , F > (f : F) -> LocalGenerator < 'a , A , T > where F : FnOnce (Scope < A , T >) -> T + 'a , T : 'a , A : 'a , { Self :: new_scoped_opt_local (DEFAULT_STACK_SIZE , f) } # [doc = " create a scoped generator with specified stack size"] pub fn new_scoped_opt < 'a , T , F > (size : usize , f : F) -> Generator < 'a , A , T > where for < 'scope > F : FnOnce (Scope < 'scope , 'a , A , T >) -> T + Send + 'a , T : Send + 'a , A : Send + 'a , { let mut gen = GeneratorImpl :: < A , T > :: new (Stack :: new (size)) ; gen . scoped_init (f) ; Generator { gen } } # [doc = " create a scoped local generator with specified stack size"] pub fn new_scoped_opt_local < 'a , T , F > (size : usize , f : F) -> LocalGenerator < 'a , A , T > where F : FnOnce (Scope < A , T >) -> T + 'a , T : 'a , A : 'a , { let mut gen = GeneratorImpl :: < A , T > :: new (Stack :: new (size)) ; gen . scoped_init (f) ; LocalGenerator { gen } } }
    };
}

impl_21!();