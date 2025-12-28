macro_rules! deps {
    () => {
        Item!();
        Literal!();
        AttrQuery!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'attr > AttrQuery < 'attr > { # [inline] pub fn tt_values (self) -> impl Iterator < Item = & 'attr crate :: tt :: TopSubtree > { self . attrs () . filter_map (| attr | attr . token_tree_value ()) } # [inline] pub fn string_value (self) -> Option < & 'attr Symbol > { self . attrs () . find_map (| attr | attr . string_value ()) } # [inline] pub fn string_value_with_span (self) -> Option < (& 'attr Symbol , span :: Span) > { self . attrs () . find_map (| attr | attr . string_value_with_span ()) } # [inline] pub fn string_value_unescape (self) -> Option < Cow < 'attr , str > > { self . attrs () . find_map (| attr | attr . string_value_unescape ()) } # [inline] pub fn exists (self) -> bool { self . attrs () . next () . is_some () } # [inline] pub fn attrs (self) -> impl Iterator < Item = & 'attr Attr > + Clone { let key = self . key ; self . attrs . iter () . filter (move | attr | attr . path . as_ident () . is_some_and (| s | * s == key)) } # [doc = " Find string value for a specific key inside token tree"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[doc(html_root_url = \"url\")]"] # [doc = "       ^^^^^^^^^^^^^ key"] # [doc = " ```"] # [inline] pub fn find_string_value_in_tt (self , key : Symbol) -> Option < & 'attr str > { self . tt_values () . find_map (| tt | { let name = tt . iter () . skip_while (| tt | ! matches ! (tt , TtElement :: Leaf (tt :: Leaf :: Ident (tt :: Ident { sym , .. })) if * sym == key)) . nth (2) ; match name { Some (TtElement :: Leaf (tt :: Leaf :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: Str | tt :: LitKind :: StrRaw (_) , .. }))) => Some (text . as_str ()) , _ => None } }) } }
    };
}

impl_29!();