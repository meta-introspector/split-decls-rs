macro_rules! deps {
    () => {
        Snippet!();
        Annotation!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'a > Snippet < 'a , Annotation < 'a > > { # [doc = " Highlight and describe a span of text within the [`source`][Self::source]"] pub fn annotation (mut self , annotation : Annotation < 'a >) -> Snippet < 'a , Annotation < 'a > > { self . markers . push (annotation) ; self } # [doc = " Highlight and describe spans of text within the [`source`][Self::source]"] pub fn annotations (mut self , annotation : impl IntoIterator < Item = Annotation < 'a > >) -> Self { self . markers . extend (annotation) ; self } }
    };
}

impl_132!()