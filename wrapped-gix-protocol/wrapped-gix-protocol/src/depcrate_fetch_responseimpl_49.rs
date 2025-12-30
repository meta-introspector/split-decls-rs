// Generated macro for impl_49 (impl)
macro_rules! Depcrate_fetch_responseimpl_49 {
() => {
// Module: crate::fetch::response
// Provides: {"impl_49"}
// Dependencies: {}
impl Acknowledgement { # [doc = " Parse an `Acknowledgement` from a `line` as received to the server."] pub fn from_line (line : & str) -> Result < Acknowledgement , Error > { let mut tokens = line . trim_end () . splitn (3 , ' ') ; match (tokens . next () , tokens . next () , tokens . next ()) { (Some (first) , id , description) => Ok (match first { "ready" => Acknowledgement :: Ready , "NAK" => Acknowledgement :: Nak , "ACK" => { let id = match id { Some (id) => gix_hash :: ObjectId :: from_hex (id . as_bytes ()) . map_err (| _ | Error :: UnknownLineType { line : line . to_owned () }) ? , None => return Err (Error :: UnknownLineType { line : line . to_owned () }) , } ; if let Some (description) = description { match description { "common" => { } "ready" => return Ok (Acknowledgement :: Ready) , _ => return Err (Error :: UnknownLineType { line : line . to_owned () }) , } } Acknowledgement :: Common (id) } _ => return Err (Error :: UnknownLineType { line : line . to_owned () }) , }) , (None , _ , _) => Err (Error :: UnknownLineType { line : line . to_owned () }) , } } # [doc = " Returns the hash of the acknowledged object if this instance acknowledges a common one."] pub fn id (& self) -> Option < & gix_hash :: ObjectId > { match self { Acknowledgement :: Common (id) => Some (id) , _ => None , } } }
};
}
