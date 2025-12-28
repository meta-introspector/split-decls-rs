macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (feature = "oprf")] impl hash2curve :: OprfParameters for NistP384 { # [doc = " See <https://www.rfc-editor.org/rfc/rfc9497.html#section-4.4-1>."] const ID : & 'static [u8] = b"P384-SHA384" ; }
    };
}

impl_46!()