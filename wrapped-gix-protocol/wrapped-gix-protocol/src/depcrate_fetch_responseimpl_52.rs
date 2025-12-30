// Generated macro for impl_52 (impl)
macro_rules! Depcrate_fetch_responseimpl_52 {
() => {
// Module: crate::fetch::response
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (any (feature = "async-client" , feature = "blocking-client"))] impl Response { # [doc = " with a friendly server, we just assume that a non-ack line is a pack line"] # [doc = " which is our hint to stop here."] fn parse_v1_ack_or_shallow_or_assume_pack (acks : & mut Vec < Acknowledgement > , shallows : & mut Vec < ShallowUpdate > , peeked_line : & str ,) -> bool { match Acknowledgement :: from_line (peeked_line) { Ok (ack) => match ack . id () { Some (id) => { if ! acks . iter () . any (| a | a . id () == Some (id)) { acks . push (ack) ; } } None => acks . push (ack) , } , Err (_) => match shallow_update_from_line (peeked_line) { Ok (shallow) => { shallows . push (shallow) ; } Err (_) => return true , } , } false } }
};
}
