// Generated macro for shallow_update_from_line (function)
macro_rules! Depcrate_fetch_responseshallow_update_from_line {
() => {
// Module: crate::fetch::response
// Provides: {"shallow_update_from_line"}
// Dependencies: {}
# [doc = " Parse a `ShallowUpdate` from a `line` as received to the server."] pub fn shallow_update_from_line (line : & str) -> Result < ShallowUpdate , Error > { match line . trim_end () . split_once (' ') { Some ((prefix , id)) => { let id = gix_hash :: ObjectId :: from_hex (id . as_bytes ()) . map_err (| _ | Error :: UnknownLineType { line : line . to_owned () }) ? ; Ok (match prefix { "shallow" => ShallowUpdate :: Shallow (id) , "unshallow" => ShallowUpdate :: Unshallow (id) , _ => return Err (Error :: UnknownLineType { line : line . to_owned () }) , }) } None => Err (Error :: UnknownLineType { line : line . to_owned () }) , } }
};
}
