macro_rules! crate_name {
    () => {
        fn crate_name (data : & ide_db :: base_db :: ExtraCrateData) -> Option < String > { data . display_name . as_ref () . map (| it | it . canonical_name () . as_str () . to_owned ()) }
    };
}

crate_name!();