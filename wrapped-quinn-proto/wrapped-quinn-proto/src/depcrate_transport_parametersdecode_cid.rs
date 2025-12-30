// Generated macro for decode_cid (function)
macro_rules! Depcrate_transport_parametersdecode_cid {
() => {
// Module: crate::transport_parameters
// Provides: {"decode_cid"}
// Dependencies: {}
fn decode_cid (len : usize , value : & mut Option < ConnectionId > , r : & mut impl Buf) -> Result < () , Error > { if len > MAX_CID_SIZE || value . is_some () || r . remaining () < len { return Err (Error :: Malformed) ; } * value = Some (ConnectionId :: from_buf (r , len)) ; Ok (()) }
};
}
