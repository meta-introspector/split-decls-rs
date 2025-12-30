// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl < const MAX_SIZE : usize > TryFrom < & ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { type Error = Error ; fn try_from (oid_ref : & ObjectIdentifierRef) -> Result < Self > { let len = oid_ref . as_bytes () . len () ; if len > MAX_SIZE { return Err (Error :: Length) ; } let mut bytes = [0u8 ; MAX_SIZE] ; bytes [.. len] . copy_from_slice (oid_ref . as_bytes ()) ; let ber = Buffer { bytes , length : len as u8 , } ; Ok (Self { ber }) } }
};
}
