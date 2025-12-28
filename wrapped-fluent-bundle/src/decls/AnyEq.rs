macro_rules! AnyEq {
    () => {
        pub trait AnyEq : Any + 'static { fn equals (& self , other : & dyn Any) -> bool ; fn as_any (& self) -> & dyn Any ; }
    };
}

AnyEq!();