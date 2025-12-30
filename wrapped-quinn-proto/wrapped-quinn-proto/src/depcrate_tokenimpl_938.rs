// Generated macro for impl_938 (impl)
macro_rules! Depcrate_tokenimpl_938 {
() => {
// Module: crate::token
// Provides: {"impl_938"}
// Dependencies: {}
impl IncomingToken { # [doc = " Construct for an `Incoming` given the first packet header, or error if the connection"] # [doc = " cannot be established"] pub (crate) fn from_header (header : & InitialHeader , server_config : & ServerConfig , remote_address : SocketAddr ,) -> Result < Self , InvalidRetryTokenError > { let unvalidated = Self { retry_src_cid : None , orig_dst_cid : header . dst_cid , validated : false , } ; if header . token . is_empty () { return Ok (unvalidated) ; } let Some (retry) = Token :: decode (& * server_config . token_key , & header . token) else { return Ok (unvalidated) ; } ; match retry . payload { TokenPayload :: Retry { address , orig_dst_cid , issued , } => { if address != remote_address { return Err (InvalidRetryTokenError) ; } if issued + server_config . retry_token_lifetime < server_config . time_source . now () { return Err (InvalidRetryTokenError) ; } Ok (Self { retry_src_cid : Some (header . dst_cid) , orig_dst_cid , validated : true , }) } TokenPayload :: Validation { ip , issued } => { if ip != remote_address . ip () { return Ok (unvalidated) ; } if issued + server_config . validation_token . lifetime < server_config . time_source . now () { return Ok (unvalidated) ; } if server_config . validation_token . log . check_and_insert (retry . nonce , issued , server_config . validation_token . lifetime) . is_err () { return Ok (unvalidated) ; } Ok (Self { retry_src_cid : None , orig_dst_cid : header . dst_cid , validated : true , }) } } } }
};
}
