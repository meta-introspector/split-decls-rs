macro_rules! deps {
    () => {
        PathParser!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < 'a > PathParser < 'a > { pub fn get_attribute_path (& self) -> hir :: AttrPath { AttrPath { segments : self . segments () . copied () . collect :: < Vec < _ > > () . into_boxed_slice () , span : self . span () , } } pub fn segments (& 'a self) -> impl Iterator < Item = & 'a Ident > { self . 0 . segments . iter () . map (| seg | & seg . ident) } pub fn span (& self) -> Span { self . 0 . span } pub fn len (& self) -> usize { self . 0 . segments . len () } pub fn segments_is (& self , segments : & [Symbol]) -> bool { self . len () == segments . len () && self . segments () . zip (segments) . all (| (a , b) | a . name == * b) } pub fn word (& self) -> Option < Ident > { (self . len () == 1) . then (| | * * self . segments () . next () . as_ref () . unwrap ()) } pub fn word_sym (& self) -> Option < Symbol > { self . word () . map (| ident | ident . name) } # [doc = " Asserts that this MetaItem is some specific word."] # [doc = ""] # [doc = " See [`word`](Self::word) for examples of what a word is."] pub fn word_is (& self , sym : Symbol) -> bool { self . word () . map (| i | i . name == sym) . unwrap_or (false) } # [doc = " Checks whether the first segments match the givens."] # [doc = ""] # [doc = " Unlike [`segments_is`](Self::segments_is),"] # [doc = " `self` may contain more segments than the number matched  against."] pub fn starts_with (& self , segments : & [Symbol]) -> bool { segments . len () < self . len () && self . segments () . zip (segments) . all (| (a , b) | a . name == * b) } }
    };
}

impl_293!();