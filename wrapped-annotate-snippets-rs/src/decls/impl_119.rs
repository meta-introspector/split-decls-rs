macro_rules! deps {
    () => {
        Title!();
        Group!();
        Element!();
        Level!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a > Group < 'a > { # [doc = " Create group with a [`Title`], deriving [`AnnotationKind::Primary`] from its [`Level`]"] pub fn with_title (title : Title < 'a >) -> Self { let level = title . level . clone () ; let mut x = Self :: with_level (level) ; x . title = Some (title) ; x } # [doc = " Create a title-less group with a primary [`Level`] for [`AnnotationKind::Primary`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/elide_header.rs")] # [doc = " ```"] # [doc = include_str ! ("../examples/elide_header.svg")] pub fn with_level (level : Level < 'a >) -> Self { Self { primary_level : level , title : None , elements : vec ! [] , } } # [doc = " Append an [`Element`] that adds context to the [`Title`]"] pub fn element (mut self , section : impl Into < Element < 'a > >) -> Self { self . elements . push (section . into ()) ; self } # [doc = " Append [`Element`]s that adds context to the [`Title`]"] pub fn elements (mut self , sections : impl IntoIterator < Item = impl Into < Element < 'a > > >) -> Self { self . elements . extend (sections . into_iter () . map (Into :: into)) ; self } pub fn is_empty (& self) -> bool { self . elements . is_empty () && self . title . is_none () } }
    };
}

impl_119!()