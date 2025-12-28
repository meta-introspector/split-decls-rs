macro_rules! deps {
    () => {
        Annotation!();
        Snippet!();
        AnnotationKind!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl AnnotationKind { # [doc = " Annotate a byte span within [`Snippet`]"] pub fn span < 'a > (self , span : Range < usize >) -> Annotation < 'a > { Annotation { span , label : None , kind : self , highlight_source : false , } } pub (crate) fn is_primary (& self) -> bool { matches ! (self , AnnotationKind :: Primary) } }
    };
}

impl_137!()