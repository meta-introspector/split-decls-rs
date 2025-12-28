macro_rules! deps {
    () => {
        NonMacroAttrKind!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl NonMacroAttrKind { pub fn descr (self) -> & 'static str { match self { NonMacroAttrKind :: Builtin (..) => "built-in attribute" , NonMacroAttrKind :: Tool => "tool attribute" , NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => { "derive helper attribute" } } } pub fn article (self) -> & 'static str { "a" } # [doc = " Users of some attributes cannot mark them as used, so they are considered always used."] pub fn is_used (self) -> bool { match self { NonMacroAttrKind :: Tool | NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => true , NonMacroAttrKind :: Builtin (..) => false , } } }
    };
}

impl_74!()