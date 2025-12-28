macro_rules! deps {
    () => {
        InsideParensRestConsumed!();
        Error!();
    };
}

macro_rules! parens {
    () => {
        deps!();
        fn parens (input : & [u8]) -> Result < Option < InsideParensRestConsumed < '_ > > , Error > { if input . first () != Some (& b'{') { return Ok (None) ; } let mut open_braces = 0 ; let mut ignore_next = false ; let mut skip_list = Vec :: new () ; for (idx , b) in input . iter () . enumerate () { match * b { b'{' => { if ignore_next { ignore_next = false ; } else { open_braces += 1 ; } } b'}' => { if ignore_next { ignore_next = false ; } else { open_braces -= 1 ; } } b'\\' => { skip_list . push (idx) ; if ignore_next { skip_list . pop () ; ignore_next = false ; } else { ignore_next = true ; } } _ => { if ignore_next { skip_list . pop () ; } ignore_next = false ; } } if open_braces == 0 { let inner : std :: borrow :: Cow < '_ , _ > = if skip_list . is_empty () { input [1 .. idx] . as_bstr () . into () } else { let mut from = 1 ; let mut buf = BString :: default () ; for next in skip_list . into_iter () { buf . push_str (& input [from .. next]) ; from = next + 1 ; } if let Some (rest) = input . get (from .. idx) { buf . push_str (rest) ; } buf . into () } ; return Ok (Some ((inner , input [idx + 1 ..] . as_bstr () , idx + 1))) ; } } Err (Error :: UnclosedBracePair { input : input . into () }) }
    };
}

parens!()