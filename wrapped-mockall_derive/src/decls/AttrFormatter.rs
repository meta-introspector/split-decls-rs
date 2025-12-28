macro_rules! AttrFormatter {
    () => {
        struct AttrFormatter < 'a > { attrs : & 'a [Attribute] , async_trait : bool , trait_variant : bool , doc : bool , must_use : bool , }
    };
}

AttrFormatter!();