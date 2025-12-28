macro_rules! deps {
    () => {
        CargoConfigFileReader!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a > CargoConfigFileReader < 'a > { fn new (toml_str : & 'a str) -> Option < Self > { let toml = DeTable :: parse (toml_str) . inspect_err (| err | tracing :: debug ! ("Failed to parse cargo config into toml: {err:?}")) . ok () ? ; let mut last_line_end = 0 ; let line_ends = toml_str . lines () . map (| l | { last_line_end += l . len () + 1 ; last_line_end }) . collect () ; Some (CargoConfigFileReader { toml_str , table : toml , line_ends }) } pub (crate) fn get_spanned (& self , accessor : impl IntoIterator < Item = & 'a str > ,) -> Option < & Spanned < DeValue < 'a > > > { let mut keys = accessor . into_iter () ; let mut val = self . table . get_ref () . get (keys . next () ?) ? ; for key in keys { let DeValue :: Table (map) = val . get_ref () else { return None } ; val = map . get (key) ? ; } Some (val) } pub (crate) fn get (& self , accessor : impl IntoIterator < Item = & 'a str >) -> Option < & DeValue < 'a > > { self . get_spanned (accessor) . map (| it | it . as_ref ()) } pub (crate) fn get_origin_root (& self , spanned : & Spanned < DeValue < 'a > >) -> Option < & AbsPath > { let span = spanned . span () ; for & line_end in & self . line_ends { if line_end < span . end { continue ; } let after_span = & self . toml_str [span . end .. line_end] ; let origin_path = after_span . strip_prefix ([',']) . unwrap_or (after_span) . trim_start () . strip_prefix (['#']) . and_then (| path | { let path = path . trim () ; if path . starts_with ("environment variable") || path . starts_with ("--config cli option") { None } else { Some (path) } }) ; return origin_path . and_then (| path | { < & Utf8Path > :: from (path) . try_into () . ok () . and_then (AbsPath :: parent) . and_then (AbsPath :: parent) }) ; } None } }
    };
}

impl_36!();