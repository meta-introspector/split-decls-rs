macro_rules! inner {
    () => {
        # [cfg (not (feature = "string"))] pub (crate) mod inner { # [derive (Clone)] pub (crate) struct Inner (pub (crate) & 'static str) ; impl Inner { pub (crate) fn from_static_ref (name : & 'static str) -> Self { Self (name) } pub (crate) fn as_str (& self) -> & str { self . 0 } pub (crate) fn into_string (self) -> String { self . as_str () . to_owned () } } }
    };
}

inner!();