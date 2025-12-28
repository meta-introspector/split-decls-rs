macro_rules! AnnotationSnippet {
    () => {
        pub enum AnnotationSnippet { # [doc = " Place a tabstop before an element"] Before , # [doc = " Place a tabstop before an element"] After , # [doc = " Place a placeholder snippet in place of the element(s)"] Over , }
    };
}

AnnotationSnippet!()