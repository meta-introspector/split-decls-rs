macro_rules! deps {
    () => {
        LineAnnotationType!();
        LineAnnotation!();
        Loc!();
        MultilineAnnotation!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < 'a > MultilineAnnotation < 'a > { pub (crate) fn increase_depth (& mut self) { self . depth += 1 ; } # [doc = " Compare two `MultilineAnnotation`s considering only the `Span` they cover."] pub (crate) fn same_span (& self , other : & MultilineAnnotation < '_ >) -> bool { self . start == other . start && self . end == other . end } pub (crate) fn as_start (& self) -> LineAnnotation < 'a > { LineAnnotation { start : self . start , end : Loc { line : self . start . line , char : self . start . char + 1 , display : self . start . display + 1 , byte : self . start . byte + 1 , } , kind : self . kind , label : None , annotation_type : LineAnnotationType :: MultilineStart (self . depth) , highlight_source : self . highlight_source , } } pub (crate) fn as_end (& self) -> LineAnnotation < 'a > { LineAnnotation { start : Loc { line : self . end . line , char : self . end . char . saturating_sub (1) , display : self . end . display . saturating_sub (1) , byte : self . end . byte . saturating_sub (1) , } , end : self . end , kind : self . kind , label : self . label . clone () , annotation_type : LineAnnotationType :: MultilineEnd (self . depth) , highlight_source : self . highlight_source , } } pub (crate) fn as_line (& self) -> LineAnnotation < 'a > { LineAnnotation { start : Loc :: default () , end : Loc :: default () , kind : self . kind , label : None , annotation_type : LineAnnotationType :: MultilineLine (self . depth) , highlight_source : self . highlight_source , } } }
    };
}

impl_60!();