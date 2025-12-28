macro_rules! deps {
    () => {
        ErrorKind!();
        Result!();
    };
}

macro_rules! validate_feature_name {
    () => {
        deps!();
        pub (crate) fn validate_feature_name (name : & str) -> Result < () > { let what = "feature name" ; if name . is_empty () { return Err (ErrorKind :: Empty (what) . into ()) ; } if name . starts_with ("dep:") { return Err (ErrorKind :: FeatureNameStartsWithDepColon (name . into ()) . into ()) ; } if name . contains ('/') { return Err (ErrorKind :: InvalidCharacter { ch : '/' , what , name : name . into () , reason : "feature name is not allowed to contain slashes" , } . into ()) ; } let mut chars = name . chars () ; if let Some (ch) = chars . next () { if ! (unicode_ident :: is_xid_start (ch) || ch == '_' || ch . is_digit (10)) { return Err (ErrorKind :: InvalidCharacter { ch , what , name : name . into () , reason : "the first character must be a Unicode XID start character or digit \
                 (most letters or `_` or `0` to `9`)" , } . into ()) ; } } for ch in chars { if ! (unicode_ident :: is_xid_continue (ch) || ch == '-' || ch == '+' || ch == '.') { return Err (ErrorKind :: InvalidCharacter { ch , what , name : name . into () , reason : "characters must be Unicode XID characters, '-', `+`, or `.` \
                 (numbers, `+`, `-`, `_`, `.`, or most letters)" , } . into ()) ; } } Ok (()) }
    };
}

validate_feature_name!();