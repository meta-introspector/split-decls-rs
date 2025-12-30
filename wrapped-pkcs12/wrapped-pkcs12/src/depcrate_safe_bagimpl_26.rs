// Generated macro for impl_26 (impl)
macro_rules! Depcrate_safe_bagimpl_26 {
() => {
// Module: crate::safe_bag
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > :: der :: DecodeValue < 'a > for SafeBag { type Error = der :: Error ; fn decode_value < R : :: der :: Reader < 'a > > (reader : & mut R , _header : :: der :: Header ,) -> :: der :: Result < Self > { let bag_id = reader . decode () ? ; let bag_value = match reader . tlv_bytes () { Ok (v) => v . to_vec () , Err (e) => return Err (e) , } ; let bag_attributes = reader . decode () ? ; Ok (Self { bag_id , bag_value , bag_attributes , }) } }
};
}
