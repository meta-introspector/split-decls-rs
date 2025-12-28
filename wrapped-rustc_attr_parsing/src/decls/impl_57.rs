macro_rules! deps {
    () => {
        LimitInvalid!();
        NameValueParser!();
        Stage!();
        AcceptContext!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < S : Stage > AcceptContext < '_ , '_ , S > { fn parse_limit_int (& self , nv : & NameValueParser) -> Option < Limit > { let Some (limit) = nv . value_as_str () else { self . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return None ; } ; let error_str = match limit . as_str () . parse () { Ok (i) => return Some (Limit :: new (i)) , Err (e) => match e . kind () { IntErrorKind :: PosOverflow => "`limit` is too large" , IntErrorKind :: Empty => "`limit` must be a non-negative integer" , IntErrorKind :: InvalidDigit => "not a valid integer" , IntErrorKind :: NegOverflow => { panic ! ("`limit` should never negatively overflow since we're parsing into a usize and we'd get Empty instead") } IntErrorKind :: Zero => { panic ! ("zero is a valid `limit` so should have returned Ok() when parsing") } kind => panic ! ("unimplemented IntErrorKind variant: {:?}" , kind) , } , } ; self . emit_err (LimitInvalid { span : self . attr_span , value_span : nv . value_span , error_str }) ; None } }
    };
}

impl_57!()