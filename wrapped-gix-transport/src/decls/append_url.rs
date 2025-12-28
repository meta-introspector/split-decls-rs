macro_rules! append_url {
    () => {
        fn append_url (base : & str , suffix : & str) -> String { let mut buf = base . to_owned () ; if base . as_bytes () . last () != Some (& b'/') { buf . push ('/') ; } buf . push_str (suffix) ; buf }
    };
}

append_url!();