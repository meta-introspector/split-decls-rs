macro_rules! escape_subsection {
    () => {
        fn escape_subsection (name : & BStr) -> Cow < '_ , BStr > { if name . find_byteset (b"\\\"") . is_none () { return name . into () ; } let mut buf = Vec :: with_capacity (name . len ()) ; for b in name . iter () . copied () { match b { b'\\' => buf . push_str (br"\\") , b'"' => buf . push_str (br#"\""#) , _ => buf . push (b) , } } BString :: from (buf) . into () }
    };
}

escape_subsection!()