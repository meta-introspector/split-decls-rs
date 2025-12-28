macro_rules! deps {
    () => {
        ExpandDatabase!();
        Attr!();
    };
}

macro_rules! check_cfg_attr {
    () => {
        deps!();
        fn check_cfg_attr (db : & dyn ExpandDatabase , attr : & Attr , krate : Crate) -> Option < bool > { if ! attr . simple_name () . as_deref () . map (| v | v == "cfg_attr") ? { return None ; } check_cfg_attr_value (db , & attr . token_tree () ? , krate) }
    };
}

check_cfg_attr!()