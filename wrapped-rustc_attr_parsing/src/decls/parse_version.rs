macro_rules! parse_version {
    () => {
        # [doc = " Parse a rustc version number written inside string literal in an attribute,"] # [doc = " like appears in `since = \"1.0.0\"`. Suffixes like \"-dev\" and \"-nightly\" are"] # [doc = " not accepted in this position, unlike when parsing CFG_RELEASE."] pub fn parse_version (s : Symbol) -> Option < RustcVersion > { let mut components = s . as_str () . split ('-') ; let d = components . next () ? ; if components . next () . is_some () { return None ; } let mut digits = d . splitn (3 , '.') ; let major = digits . next () ? . parse () . ok () ? ; let minor = digits . next () ? . parse () . ok () ? ; let patch = digits . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) }
    };
}

parse_version!()