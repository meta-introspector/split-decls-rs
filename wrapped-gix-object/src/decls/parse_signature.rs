macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! parse_signature {
    () => {
        deps!();
        pub (crate) fn parse_signature (raw : & BStr) -> Result < gix_actor :: SignatureRef < '_ > , crate :: decode :: Error > { gix_actor :: SignatureRef :: from_bytes :: < crate :: decode :: ParseError > (raw . as_ref ()) . map_err (| err | crate :: decode :: Error :: with_err (err , raw . as_ref ())) }
    };
}

parse_signature!();