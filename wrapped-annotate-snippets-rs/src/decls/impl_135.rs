macro_rules! deps {
    () => {
        AnnotationKind!();
        OptionCow!();
        Annotation!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'a > Annotation < 'a > { # [doc = " Describe the reason the span is highlighted"] # [doc = ""] # [doc = " This will be styled according to the [`AnnotationKind`]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Pre-styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn label (mut self , label : impl Into < OptionCow < 'a > >) -> Self { self . label = label . into () . 0 ; self } # [doc = " Style the source according to the [`AnnotationKind`]"] # [doc = ""] # [doc = " This gives extra emphasis to this annotation"] pub fn highlight_source (mut self , highlight_source : bool) -> Self { self . highlight_source = highlight_source ; self } }
    };
}

impl_135!();