// Generated macro for SigningKey (struct)
macro_rules! Depcrate_lms_privateSigningKey {
() => {
// Module: crate::lms::private
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " Opaque struct representing a LMS private key"] # [doc = ""] # [doc = " Note: there is no requirement to map specific LMS algorithms to specific"] # [doc = " LM-OTS algorithms so it must be parametrized. With the algorithms provided"] # [doc = " by this crate, this is done via"] # [doc = " [LmsSha256M32H10](crate::lms::LmsSha256M32H10)<[LmsOtsSha256N32W4](crate::ots::LmsOtsSha256N32W4)>."] pub struct SigningKey < Mode : LmsMode > { id : Identifier , seed : Output < Mode :: Hasher > , auth_tree : Array < Output < Mode :: Hasher > , Mode :: TreeLen > , q : u32 , }
};
}
