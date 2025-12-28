macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl fmt :: Display for SyntaxText { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . try_for_each_chunk (| chunk | fmt :: Display :: fmt (chunk , f)) } }
    };
}

impl_88!();