// Generated macro for NiceDuration (struct)
macro_rules! Depcrate_utilNiceDuration {
() => {
// Module: crate::util
// Provides: {"NiceDuration"}
// Dependencies: {}
# [doc = " A type that provides \"nicer\" Display and Serialize impls for"] # [doc = " std::time::Duration. The serialization format should actually be compatible"] # [doc = " with the Deserialize impl for std::time::Duration, since this type only"] # [doc = " adds new fields."] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub (crate) struct NiceDuration (pub time :: Duration) ;
};
}
