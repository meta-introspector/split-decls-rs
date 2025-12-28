macro_rules! parse_name_and_email {
    () => {
        fn parse_name_and_email (line : & BStr , line_number : usize ,) -> Result < (Option < & '_ BStr > , Option < & '_ BStr > , & '_ BStr) , Error > { match line . find_byte (b'<') { Some (start_bracket) => { let email = & line [start_bracket + 1 ..] ; let closing_bracket = email . find_byte (b'>') . ok_or_else (| | Error :: Malformed { line_number , line : line . into () , message : "Missing closing bracket '>' in email" . into () , }) ? ; let email = email [.. closing_bracket] . trim () . as_bstr () ; if email . is_empty () { return Err (Error :: Malformed { line_number , line : line . into () , message : "Email must not be empty" . into () , }) ; } let name = line [.. start_bracket] . trim () . as_bstr () ; let rest = line [start_bracket + closing_bracket + 2 ..] . as_bstr () ; Ok (((! name . is_empty ()) . then_some (name) , Some (email) , rest)) } None => Ok ((None , None , line)) , } }
    };
}

parse_name_and_email!()