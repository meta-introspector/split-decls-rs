macro_rules! deps {
    () => {
        Boolean!();
        Error!();
        Repository!();
    };
}

macro_rules! extract_drivers {
    () => {
        deps!();
        # [doc = " Obtain a list of all configured driver, but ignore those in sections that we don't trust enough."] fn extract_drivers (repo : & Repository) -> Result < Vec < gix_filter :: Driver > , pipeline :: options :: Error > { repo . config . resolved . sections_by_name ("filter") . into_iter () . flatten () . filter (| s | repo . filter_config_section () (s . meta ())) . filter_map (| s | { s . header () . subsection_name () . map (| name | { Ok (gix_filter :: Driver { name : name . to_owned () , clean : s . value ("clean") . map (Cow :: into_owned) , smudge : s . value ("smudge") . map (Cow :: into_owned) , process : s . value ("process") . map (Cow :: into_owned) , required : s . value ("required") . map (| value | gix_config :: Boolean :: try_from (value . as_ref ())) . transpose () . map_err (| err | pipeline :: options :: Error :: Driver { name : name . to_owned () , source : err , }) ? . unwrap_or_default () . into () , }) }) }) . collect :: < Result < Vec < _ > , pipeline :: options :: Error > > () }
    };
}

extract_drivers!()