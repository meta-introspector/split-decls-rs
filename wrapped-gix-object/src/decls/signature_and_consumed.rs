macro_rules! signature_and_consumed {
    () => {
        pub (crate) fn signature_and_consumed < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < (gix_actor :: SignatureRef < 'a > , & 'a BStr) , E > { let original = * i ; gix_actor :: signature :: decode (i) . map (| signature | { let consumed = original . len () - i . len () ; (signature , original [.. consumed] . as_bstr ()) }) }
    };
}

signature_and_consumed!()