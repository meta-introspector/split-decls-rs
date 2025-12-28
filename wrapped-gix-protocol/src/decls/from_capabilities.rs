macro_rules! deps {
    () => {
        Error!();
        InternalRef!();
    };
}

macro_rules! from_capabilities {
    () => {
        deps!();
        pub (crate) fn from_capabilities < 'a > (capabilities : impl Iterator < Item = gix_transport :: client :: capabilities :: Capability < 'a > > ,) -> Result < Vec < InternalRef > , Error > { let mut out_refs = Vec :: new () ; let symref_values = capabilities . filter_map (| c | { if c . name () == b"symref" . as_bstr () { c . value () . map (ToOwned :: to_owned) } else { None } }) ; for symref in symref_values { let (left , right) = symref . split_at (symref . find_byte (b':') . ok_or_else (| | Error :: MalformedSymref { symref : symref . to_owned () , }) ?) ; if left . is_empty () || right . is_empty () { return Err (Error :: MalformedSymref { symref : symref . to_owned () , }) ; } out_refs . push (InternalRef :: SymbolicForLookup { path : left . into () , target : match & right [1 ..] { b"(null)" => None , name => Some (name . into ()) , } , }) ; } Ok (out_refs) }
    };
}

from_capabilities!();