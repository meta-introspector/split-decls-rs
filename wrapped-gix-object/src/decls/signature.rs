macro_rules! signature {
    () => {
        pub (crate) fn signature < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < gix_actor :: SignatureRef < 'a > , E > { gix_actor :: signature :: decode (i) }
    };
}

signature!();