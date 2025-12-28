macro_rules! deps {
    () => {
        Repository!();
        Options!();
        Error!();
        Pipeline!();
        Core!();
    };
}

macro_rules! impl_902 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'repo > Pipeline < 'repo > { # [doc = " Extract options from `repo` that are needed to properly drive a standard git filter pipeline."] pub fn options (repo : & 'repo Repository) -> Result < gix_filter :: pipeline :: Options , pipeline :: options :: Error > { let config = & repo . config . resolved ; let encodings = Core :: CHECK_ROUND_TRIP_ENCODING . try_into_encodings (config . string ("core.checkRoundtripEncoding")) ? ; let safe_crlf = config . string ("core.safecrlf") . map (| value | Core :: SAFE_CRLF . try_into_safecrlf (value)) . transpose () . map (Option :: unwrap_or_default) . with_lenient_default_value (repo . config . lenient_config , gix_filter :: pipeline :: CrlfRoundTripCheck :: Fail ,) ? ; let auto_crlf = config . string ("core.autocrlf") . map (| value | Core :: AUTO_CRLF . try_into_autocrlf (value)) . transpose () . with_leniency (repo . config . lenient_config) ? . unwrap_or_default () ; let eol = config . string ("core.eol") . map (| value | Core :: EOL . try_into_eol (value)) . transpose () ? ; let drivers = extract_drivers (repo) ? ; Ok (gix_filter :: pipeline :: Options { drivers , eol_config : gix_filter :: eol :: Configuration { auto_crlf , eol } , encodings_with_roundtrip_check : encodings , crlf_roundtrip_check : safe_crlf , object_hash : repo . object_hash () , }) } # [doc = " Create a new instance by extracting all necessary information and configuration from a `repo` along with `cache` for accessing"] # [doc = " attributes. The `index` is used for some filters which may access it under very specific circumstances."] pub fn new (repo : & 'repo Repository , cache : gix_worktree :: Stack) -> Result < Self , pipeline :: options :: Error > { let pipeline = gix_filter :: Pipeline :: new (repo . command_context () ? , Self :: options (repo) ?) ; Ok (Pipeline { inner : pipeline , cache , repo , }) } # [doc = " Detach the repository and obtain the individual functional parts."] pub fn into_parts (self) -> (gix_filter :: Pipeline , gix_worktree :: Stack) { (self . inner , self . cache) } }
    };
}

impl_902!()