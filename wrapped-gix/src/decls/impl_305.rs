macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl crate :: Repository { pub (crate) fn filter_config_section (& self) -> fn (& gix_config :: file :: Metadata) -> bool { self . options . filter_config_section . unwrap_or (config :: section :: is_trusted) } fn subsection_str_names_of < 'a > (& 'a self , header_name : & 'a str) -> BTreeSet < & 'a str > { self . config . resolved . sections_by_name (header_name) . map (| it | { let filter = self . filter_config_section () ; it . filter (move | s | filter (s . meta ())) . filter_map (| section | section . header () . subsection_name () . and_then (| b | b . to_str () . ok ())) . collect () }) . unwrap_or_default () } }
    };
}

impl_305!()