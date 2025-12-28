macro_rules! deps {
    () => {
        Snippet!();
        Level!();
        OptionCow!();
        AnnotationKind!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc = " # Customize the `Level`"] impl < 'a > Level < 'a > { # [doc = " Replace the name describing this [`Level`]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Pre-styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/custom_level.rs")] # [doc = " ```"] # [doc = include_str ! ("../examples/custom_level.svg")] pub fn with_name (self , name : impl Into < OptionCow < 'a > >) -> Level < 'a > { Level { name : Some (name . into () . 0) , level : self . level , } } # [doc = " Do not show the [`Level`]s name"] # [doc = ""] # [doc = " Useful for:"] # [doc = " - Another layer of the application will include the level (e.g. when rendering errors)"] # [doc = " - [`Message`]s that are part of a previous [`Group`][crate::Group] [`Element`][crate::Element]s"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use annotate_snippets::{Group, Snippet, AnnotationKind, Level};"] # [doc = "let source = r#\"fn main() {"] # [doc = "     let b: &[u8] = include_str!(\"file.txt\");    //~ ERROR mismatched types"] # [doc = "     let s: &str = include_bytes!(\"file.txt\");   //~ ERROR mismatched types"] # [doc = " }\"#;"] # [doc = " let report = &["] # [doc = "     Level::ERROR.primary_title(\"mismatched types\").id(\"E0308\")"] # [doc = "         .element("] # [doc = "             Snippet::source(source)"] # [doc = "                 .path(\"$DIR/mismatched-types.rs\")"] # [doc = "                 .annotation("] # [doc = "                     AnnotationKind::Primary"] # [doc = "                         .span(105..131)"] # [doc = "                         .label(\"expected `&str`, found `&[u8; 0]`\"),"] # [doc = "                 )"] # [doc = "                 .annotation("] # [doc = "                     AnnotationKind::Context"] # [doc = "                         .span(98..102)"] # [doc = "                         .label(\"expected due to this\"),"] # [doc = "                 ),"] # [doc = "         )"] # [doc = "         .element("] # [doc = "             Level::NOTE"] # [doc = "                 .no_name()"] # [doc = "                 .message(\"expected reference `&str`\\nfound reference `&'static [u8; 0]`\"),"] # [doc = "         ),"] # [doc = " ];"] # [doc = " ```"] pub fn no_name (self) -> Level < 'a > { self . with_name (None :: < & str >) } }
    };
}

impl_8!();