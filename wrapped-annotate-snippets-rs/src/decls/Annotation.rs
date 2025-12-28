macro_rules! deps {
    () => {
        Snippet!();
        AnnotationKind!();
    };
}

macro_rules! Annotation {
    () => {
        deps!();
        # [doc = " Highlight and describe a span of text within a [`Snippet`]"] # [doc = ""] # [doc = " See [`AnnotationKind`] to create an annotation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/expected_type.rs")] # [doc = " ```"] # [doc = ""] # [doc = include_str ! ("../examples/expected_type.svg")] # [derive (Clone , Debug)] pub struct Annotation < 'a > { pub (crate) span : Range < usize > , pub (crate) label : Option < Cow < 'a , str > > , pub (crate) kind : AnnotationKind , pub (crate) highlight_source : bool , }
    };
}

Annotation!();