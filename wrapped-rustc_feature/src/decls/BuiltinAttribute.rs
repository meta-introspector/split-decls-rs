macro_rules! deps {
    () => {
        AttributeTemplate!();
        AttributeGate!();
        AttributeType!();
        AttributeSafety!();
        AttributeDuplicates!();
    };
}

macro_rules! BuiltinAttribute {
    () => {
        deps!();
        pub struct BuiltinAttribute { pub name : Symbol , # [doc = " Whether this attribute is encode cross crate."] # [doc = ""] # [doc = " If so, it is encoded in the crate metadata."] # [doc = " Otherwise, it can only be used in the local crate."] pub encode_cross_crate : EncodeCrossCrate , pub type_ : AttributeType , pub safety : AttributeSafety , pub template : AttributeTemplate , pub duplicates : AttributeDuplicates , pub gate : AttributeGate , }
    };
}

BuiltinAttribute!();