macro_rules! deps {
    () => {
        ClassSet!();
        Span!();
        ClassSetUnion!();
        ClassSetItem!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl ClassSet { # [doc = " Build a set from a union."] pub fn union (ast : ClassSetUnion) -> ClassSet { ClassSet :: Item (ClassSetItem :: Union (ast)) } # [doc = " Return the span of this character class set."] pub fn span (& self) -> & Span { match * self { ClassSet :: Item (ref x) => x . span () , ClassSet :: BinaryOp (ref x) => & x . span , } } # [doc = " Return true if and only if this class set is empty."] fn is_empty (& self) -> bool { match * self { ClassSet :: Item (ClassSetItem :: Empty (_)) => true , _ => false , } } }
    };
}

impl_90!();