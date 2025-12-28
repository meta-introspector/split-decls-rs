macro_rules! deps {
    () => {
        FlagsItemKind!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl FlagsItemKind { # [doc = " Returns true if and only if this item is a negation operator."] pub fn is_negation (& self) -> bool { match * self { FlagsItemKind :: Negation => true , _ => false , } } }
    };
}

impl_116!();