macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! replacement_objects_refs_prefix {
    () => {
        deps!();
        fn replacement_objects_refs_prefix (config : & gix_config :: File < 'static > , lenient : bool , mut filter_config_section : fn (& gix_config :: file :: Metadata) -> bool ,) -> Result < Option < BString > , Error > { let is_disabled = config :: shared :: is_replace_refs_enabled (config , lenient , filter_config_section) . map_err (config :: Error :: ConfigBoolean) ? . unwrap_or (true) ; if is_disabled { return Ok (None) ; } let ref_base = { let key = "gitoxide.objects.replaceRefBase" ; debug_assert_eq ! (gitoxide :: Objects :: REPLACE_REF_BASE . logical_name () , key) ; config . string_filter (key , & mut filter_config_section) . unwrap_or_else (| | Cow :: Borrowed ("refs/replace/" . into ())) } . into_owned () ; Ok (Some (ref_base)) }
    };
}

replacement_objects_refs_prefix!()