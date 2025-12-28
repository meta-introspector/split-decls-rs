macro_rules! deps {
    () => {
        Result!();
        ErrorKind!();
    };
}

macro_rules! validate_name {
    () => {
        deps!();
        pub (crate) fn validate_name (name : & str , what : & 'static str) -> Result < () > { if name . is_empty () { return Err (ErrorKind :: Empty (what) . into ()) ; } let mut chars = name . chars () ; if let Some (ch) = chars . next () { if ch . is_digit (10) { return Err (ErrorKind :: InvalidCharacter { ch , what , name : name . into () , reason : "the name cannot start with a digit" , } . into ()) ; } if ! (unicode_ident :: is_xid_start (ch) || ch == '_') { return Err (ErrorKind :: InvalidCharacter { ch , what , name : name . into () , reason : "the first character must be a Unicode XID start character \
                 (most letters or `_`)" , } . into ()) ; } } for ch in chars { if ! (unicode_ident :: is_xid_continue (ch) || ch == '-') { return Err (ErrorKind :: InvalidCharacter { ch , what , name : name . into () , reason : "characters must be Unicode XID characters \
                 (numbers, `-`, `_`, or most letters)" , } . into ()) ; } } Ok (()) }
    };
}

validate_name!();