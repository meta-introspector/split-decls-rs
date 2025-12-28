macro_rules! deps {
    () => {
        MetaItemLit!();
        LitKind!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl MetaItemLit { pub fn value_str (& self) -> Option < Symbol > { LitKind :: from_token_lit (self . as_token_lit ()) . ok () . and_then (| lit | lit . str ()) } }
    };
}

impl_293!()