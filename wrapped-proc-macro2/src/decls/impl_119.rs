macro_rules! deps {
    () => {
        Span!();
        Ident!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl Ident { # [track_caller] pub (crate) fn new_checked (string : & str , span : Span) -> Self { validate_ident (string) ; Ident :: new_unchecked (string , span) } pub (crate) fn new_unchecked (string : & str , span : Span) -> Self { Ident { sym : Box :: from (string) , span , raw : false , } } # [track_caller] pub (crate) fn new_raw_checked (string : & str , span : Span) -> Self { validate_ident_raw (string) ; Ident :: new_raw_unchecked (string , span) } pub (crate) fn new_raw_unchecked (string : & str , span : Span) -> Self { Ident { sym : Box :: from (string) , span , raw : true , } } pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn set_span (& mut self , span : Span) { self . span = span ; } }
    };
}

impl_119!()