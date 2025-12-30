// Generated macro for impl_1196 (impl)
macro_rules! Depcrate_runtime_nsobjectimpl_1196 {
() => {
// Module: crate::runtime::nsobject
// Provides: {"impl_1196"}
// Dependencies: {}
# [doc = " Hashing in Objective-C has the exact same requirement as in Rust:"] # [doc = ""] # [doc = " > If two objects are equal (as determined by the isEqual: method),"] # [doc = " > they must have the same hash value."] # [doc = ""] # [doc = " See <https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash>"] impl hash :: Hash for NSObject { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { < Self as NSObjectProtocol > :: hash (self) . hash (state) ; } }
};
}
