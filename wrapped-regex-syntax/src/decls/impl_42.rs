macro_rules! deps {
    () => {
        ClassBracketed!();
        ClassSet!();
        ClassInduct!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a > ClassInduct < 'a > { fn from_bracketed (ast : & 'a ast :: ClassBracketed) -> ClassInduct < 'a > { ClassInduct :: from_set (& ast . kind) } fn from_set (ast : & 'a ast :: ClassSet) -> ClassInduct < 'a > { match * ast { ast :: ClassSet :: Item (ref item) => ClassInduct :: Item (item) , ast :: ClassSet :: BinaryOp (ref op) => ClassInduct :: BinaryOp (op) , } } }
    };
}

impl_42!();