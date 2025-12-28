macro_rules! deps {
    () => {
        MatchKind!();
        AttributeId!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl MatchKind { # [doc = " return the id of the macro that resolved us, or `None` if that didn't happen."] pub fn source_id (& self) -> Option < AttributeId > { match self { MatchKind :: Attribute { macro_id : id } | MatchKind :: Macro { parent_macro_id : id } => * id , } } }
    };
}

impl_43!()