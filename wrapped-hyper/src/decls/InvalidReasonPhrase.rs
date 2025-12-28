macro_rules! deps {
    () => {
        Error!();
        ReasonPhrase!();
    };
}

macro_rules! InvalidReasonPhrase {
    () => {
        deps!();
        # [doc = " Error indicating an invalid byte when constructing a `ReasonPhrase`."] # [doc = ""] # [doc = " See [the spec][spec] for details on allowed bytes."] # [doc = ""] # [doc = " [spec]: https://httpwg.org/http-core/draft-ietf-httpbis-messaging-latest.html#rfc.section.4.p.7"] # [derive (Debug)] pub struct InvalidReasonPhrase { bad_byte : u8 , }
    };
}

InvalidReasonPhrase!()