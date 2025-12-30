// Generated macro for allocating (module)
macro_rules! Depcrate_private_key_infoallocating {
() => {
// Module: crate::private_key_info
// Provides: {"allocating"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod allocating { use super :: * ; use alloc :: borrow :: ToOwned ; use core :: borrow :: Borrow ; use der :: referenced :: * ; impl BitStringLike for BitString { fn as_bit_string (& self) -> BitStringRef < '_ > { BitStringRef :: from (self) } } impl < 'a > RefToOwned < 'a > for PrivateKeyInfoRef < 'a > { type Owned = PrivateKeyInfoOwned ; fn ref_to_owned (& self) -> Self :: Owned { PrivateKeyInfoOwned { algorithm : self . algorithm . ref_to_owned () , private_key : self . private_key . to_owned () , public_key : self . public_key . ref_to_owned () , } } } impl OwnedToRef for PrivateKeyInfoOwned { type Borrowed < 'a > = PrivateKeyInfoRef < 'a > ; fn owned_to_ref (& self) -> Self :: Borrowed < '_ > { PrivateKeyInfoRef { algorithm : self . algorithm . owned_to_ref () , private_key : self . private_key . borrow () , public_key : self . public_key . owned_to_ref () , } } } }
};
}
