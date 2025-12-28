macro_rules! deps {
    () => {
        InternalRef!();
        Shallow!();
        Error!();
    };
}

macro_rules! parse_v1 {
    () => {
        deps!();
        pub (in crate :: handshake :: refs) fn parse_v1 (num_initial_out_refs : usize , out_refs : & mut Vec < InternalRef > , out_shallow : & mut Vec < ShallowUpdate > , line : & BStr ,) -> Result < () , Error > { let trimmed = line . trim_end () ; let (hex_hash , path) = trimmed . split_at (trimmed . find (b" ") . ok_or_else (| | Error :: MalformedV1RefLine (trimmed . to_owned () . into ())) ? ,) ; let path = & path [1 ..] ; if path . is_empty () { return Err (Error :: MalformedV1RefLine (trimmed . to_owned () . into ())) ; } match path . strip_suffix (b"^{}") { Some (stripped) => { if hex_hash . iter () . all (| b | * b == b'0') && stripped == b"capabilities" { return Ok (()) ; } let (previous_path , tag) = out_refs . pop () . and_then (InternalRef :: unpack_direct) . ok_or (Error :: InvariantViolation { message : "Expecting peeled refs to be preceded by direct refs" , }) ? ; if previous_path != stripped { return Err (Error :: InvariantViolation { message : "Expecting peeled refs to have the same base path as the previous, unpeeled one" , }) ; } out_refs . push (InternalRef :: Peeled { path : previous_path , tag , object : gix_hash :: ObjectId :: from_hex (hex_hash . as_bytes ()) ? , }) ; } None => { let object = match gix_hash :: ObjectId :: from_hex (hex_hash . as_bytes ()) { Ok (id) => id , Err (_) if hex_hash . as_bstr () == "shallow" => { let id = gix_hash :: ObjectId :: from_hex (path) ? ; out_shallow . push (ShallowUpdate :: Shallow (id)) ; return Ok (()) ; } Err (err) => return Err (err . into ()) , } ; match out_refs . iter () . take (num_initial_out_refs) . position (| r | r . lookup_symbol_has_path (path . into ())) { Some (position) => match out_refs . swap_remove (position) { InternalRef :: SymbolicForLookup { path : _ , target } => out_refs . push (InternalRef :: Symbolic { path : path . into () , tag : None , object , target , }) , _ => unreachable ! ("Bug in lookup_symbol_has_path - must return lookup symbols") , } , None => out_refs . push (InternalRef :: Direct { object , path : path . into () , }) , } } } Ok (()) }
    };
}

parse_v1!();