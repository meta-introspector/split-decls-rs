macro_rules! deps {
    () => {
        ExpandDatabase!();
        Attr!();
    };
}

macro_rules! check_cfg {
    () => {
        deps!();
        fn check_cfg (db : & dyn ExpandDatabase , attr : & Attr , krate : Crate) -> Option < bool > { if ! attr . simple_name () . as_deref () . map (| v | v == "cfg") ? { return None ; } let cfg = parse_from_attr_token_tree (& attr . meta () ? . token_tree () ?) ? ; let enabled = krate . cfg_options (db) . check (& cfg) != Some (false) ; Some (enabled) }
    };
}

check_cfg!()