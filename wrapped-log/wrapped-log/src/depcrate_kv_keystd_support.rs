// Generated macro for std_support (module)
macro_rules! Depcrate_kv_keystd_support {
() => {
// Module: crate::kv::key
// Provides: {"std_support"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_support { use super :: * ; use std :: borrow :: Cow ; impl ToKey for String { fn to_key (& self) -> Key < '_ > { Key :: from_str (self) } } impl < 'a > ToKey for Cow < 'a , str > { fn to_key (& self) -> Key < '_ > { Key :: from_str (self) } } }
};
}
