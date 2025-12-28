macro_rules! deps {
    () => {
        AssocItem!();
        Item!();
        ForeignItem!();
    };
}

macro_rules! macro_257 {
    () => {
        deps!();
        impl_has_attrs ! (const SUPPORTS_CUSTOM_INNER_ATTRS : bool = true , AssocItem , ForeignItem , Item ,) ;
    };
}

macro_257!()