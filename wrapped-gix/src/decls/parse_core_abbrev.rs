macro_rules! deps {
    () => {
        Error!();
        Core!();
        Kind!();
    };
}

macro_rules! parse_core_abbrev {
    () => {
        deps!();
        pub (crate) fn parse_core_abbrev (config : & gix_config :: File < 'static > , object_hash : gix_hash :: Kind ,) -> Result < Option < usize > , Error > { Ok (config . string ("core.abbrev") . map (| abbrev | Core :: ABBREV . try_into_abbreviation (abbrev , object_hash)) . transpose () ? . flatten ()) }
    };
}

parse_core_abbrev!()