macro_rules! deps {
    () => {
        ClassUnicodeOpKind!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl ClassUnicodeOpKind { # [doc = " Whether the op is an equality op or not."] pub fn is_equal (& self) -> bool { match * self { ClassUnicodeOpKind :: Equal | ClassUnicodeOpKind :: Colon => true , _ => false , } } }
    };
}

impl_87!();