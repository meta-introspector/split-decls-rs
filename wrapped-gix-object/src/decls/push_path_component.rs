macro_rules! push_path_component {
    () => {
        fn push_path_component (base : & mut BString , component : & [u8]) -> usize { let prev_len = base . len () ; debug_assert ! (base . last () != Some (& b'/')) ; if ! base . is_empty () { base . push_byte (b'/') ; } base . push_str (component) ; prev_len }
    };
}

push_path_component!()