macro_rules! deps {
    () => {
        AttrFormatter!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a > AttrFormatter < 'a > { fn new (attrs : & 'a [Attribute]) -> AttrFormatter < 'a > { Self { attrs , async_trait : true , trait_variant : false , doc : true , must_use : false , } } fn async_trait (& mut self , allowed : bool) -> & mut Self { self . async_trait = allowed ; self } fn trait_variant (& mut self , allowed : bool) -> & mut Self { self . trait_variant = allowed ; self } fn doc (& mut self , allowed : bool) -> & mut Self { self . doc = allowed ; self } fn must_use (& mut self , allowed : bool) -> & mut Self { self . must_use = allowed ; self } # [allow (clippy :: needless_bool)] # [allow (clippy :: if_same_then_else)] fn format (& mut self) -> Vec < Attribute > { self . attrs . iter () . filter (| attr | { let i = attr . path () . segments . last () . map (| ps | & ps . ident) ; if is_concretize (attr) { false } else if i . is_none () { false } else if * i . as_ref () . unwrap () == "derive" { false } else if * i . as_ref () . unwrap () == "doc" { self . doc } else if * i . as_ref () . unwrap () == "async_trait" { self . async_trait } else if * i . as_ref () . unwrap () == "make" { self . trait_variant } else if * i . as_ref () . unwrap () == "expect" { false } else if * i . as_ref () . unwrap () == "inline" { false } else if * i . as_ref () . unwrap () == "cold" { false } else if * i . as_ref () . unwrap () == "instrument" { false } else if * i . as_ref () . unwrap () == "link_name" { false } else if * i . as_ref () . unwrap () == "must_use" { self . must_use } else if * i . as_ref () . unwrap () == "auto_enum" { false } else { true } }) . cloned () . collect () } }
    };
}

impl_131!();