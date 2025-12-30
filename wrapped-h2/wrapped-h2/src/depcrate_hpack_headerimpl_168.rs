// Generated macro for impl_168 (impl)
macro_rules! Depcrate_hpack_headerimpl_168 {
() => {
// Module: crate::hpack::header
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > Name < 'a > { pub fn into_entry (self , value : Bytes) -> Result < Header , DecoderError > { match self { Name :: Field (name) => Ok (Header :: Field { name : name . clone () , value : HeaderValue :: from_bytes (& value) ? , }) , Name :: Authority => Ok (Header :: Authority (BytesStr :: try_from (value) ?)) , Name :: Method => Ok (Header :: Method (Method :: from_bytes (& value) ?)) , Name :: Scheme => Ok (Header :: Scheme (BytesStr :: try_from (value) ?)) , Name :: Path => Ok (Header :: Path (BytesStr :: try_from (value) ?)) , Name :: Protocol => Ok (Header :: Protocol (Protocol :: try_from (value) ?)) , Name :: Status => { match StatusCode :: from_bytes (& value) { Ok (status) => Ok (Header :: Status (status)) , Err (_) => Err (DecoderError :: InvalidStatusCode) , } } } } pub fn as_slice (& self) -> & [u8] { match * self { Name :: Field (ref name) => name . as_ref () , Name :: Authority => b":authority" , Name :: Method => b":method" , Name :: Scheme => b":scheme" , Name :: Path => b":path" , Name :: Protocol => b":protocol" , Name :: Status => b":status" , } } }
};
}
