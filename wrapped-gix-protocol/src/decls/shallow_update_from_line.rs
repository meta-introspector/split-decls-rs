macro_rules! deps {
    () => {
        Error!();
        Shallow!();
    };
}

macro_rules! shallow_update_from_line {
    () => {
        deps!();
        # [doc = " Parse a `ShallowUpdate` from a `line` as received to the server."] pub fn shallow_update_from_line (line : & str) -> Result < ShallowUpdate , Error > { match line . trim_end () . split_once (' ') { Some ((prefix , id)) => { let id = gix_hash :: ObjectId :: from_hex (id . as_bytes ()) . map_err (| _ | Error :: UnknownLineType { line : line . to_owned () }) ? ; Ok (match prefix { "shallow" => ShallowUpdate :: Shallow (id) , "unshallow" => ShallowUpdate :: Unshallow (id) , _ => return Err (Error :: UnknownLineType { line : line . to_owned () }) , }) } None => Err (Error :: UnknownLineType { line : line . to_owned () }) , } }
    };
}

shallow_update_from_line!();