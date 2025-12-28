macro_rules! deps {
    () => {
        AttrKind!();
        Deprecation!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Deprecation { fn attribute (version : & 'static str , old : AttrKind , new : AttrKind , span : Span) -> Self { Self { span , id : "old_attribute" , version , description : format ! ("Attribute `#[{}(...)]` has been deprecated in favor of `#[{}(...)]`" , old . as_str () , new . as_str ()) , } } }
    };
}

impl_67!();