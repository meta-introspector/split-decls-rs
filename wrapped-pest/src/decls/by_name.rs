macro_rules! by_name {
    () => {
        pub fn by_name (name : & str) -> Option < Box < dyn Fn (char) -> bool > > { for property in binary :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } for property in category :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } for property in script :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } None }
    };
}

by_name!();