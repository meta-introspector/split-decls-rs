macro_rules! Attr {
    () => {
        # [doc = " A single automock attribute"] # [allow (clippy :: large_enum_variant)] enum Attr { Target (Ident) , Type (TraitItemType) , }
    };
}

Attr!();