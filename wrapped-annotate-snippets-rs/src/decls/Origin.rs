macro_rules! deps {
    () => {
        Element!();
        AnnotationKind!();
        Group!();
        Snippet!();
        Level!();
    };
}

macro_rules! Origin {
    () => {
        deps!();
        # [doc = " A source location [`Element`] in a [`Group`]"] # [doc = ""] # [doc = " If you have source available, see instead [`Snippet`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use annotate_snippets::{Group, Snippet, AnnotationKind, Level, Origin};"] # [doc = " let report = &["] # [doc = "     Level::ERROR.primary_title(\"mismatched types\").id(\"E0308\")"] # [doc = "         .element("] # [doc = "             Origin::path(\"$DIR/mismatched-types.rs\")"] # [doc = "         )"] # [doc = " ];"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Origin < 'a > { pub (crate) path : Cow < 'a , str > , pub (crate) line : Option < usize > , pub (crate) char_column : Option < usize > , }
    };
}

Origin!()