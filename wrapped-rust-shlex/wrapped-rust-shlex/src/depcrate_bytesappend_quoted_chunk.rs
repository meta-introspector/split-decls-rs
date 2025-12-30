// Generated macro for append_quoted_chunk (function)
macro_rules! Depcrate_bytesappend_quoted_chunk {
() => {
// Module: crate::bytes
// Provides: {"append_quoted_chunk"}
// Dependencies: {}
fn append_quoted_chunk (out : & mut Vec < u8 > , cur_chunk : & [u8] , strategy : QuotingStrategy) { match strategy { QuotingStrategy :: Unquoted => { out . extend_from_slice (cur_chunk) ; } , QuotingStrategy :: SingleQuoted => { out . reserve (cur_chunk . len () + 2) ; out . push (b'\'') ; out . extend_from_slice (cur_chunk) ; out . push (b'\'') ; } , QuotingStrategy :: DoubleQuoted => { out . reserve (cur_chunk . len () + 2) ; out . push (b'"') ; for & c in cur_chunk . into_iter () { if let b'$' | b'`' | b'"' | b'\\' = c { out . push (b'\\') ; } out . push (c) ; } out . push (b'"') ; } , } }
};
}
