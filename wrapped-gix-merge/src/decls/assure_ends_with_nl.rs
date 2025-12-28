macro_rules! assure_ends_with_nl {
    () => {
        pub fn assure_ends_with_nl (out : & mut Vec < u8 > , nl : & BStr) { if ! out . is_empty () && ! out . ends_with (b"\n") { out . push_str (nl) ; } }
    };
}

assure_ends_with_nl!();