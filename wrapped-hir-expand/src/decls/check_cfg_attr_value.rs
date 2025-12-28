macro_rules! deps {
    () => {
        ExpandDatabase!();
    };
}

macro_rules! check_cfg_attr_value {
    () => {
        deps!();
        pub fn check_cfg_attr_value (db : & dyn ExpandDatabase , attr : & TokenTree , krate : Crate ,) -> Option < bool > { let cfg_expr = parse_from_attr_token_tree (attr) ? ; let enabled = krate . cfg_options (db) . check (& cfg_expr) != Some (false) ; Some (enabled) }
    };
}

check_cfg_attr_value!()