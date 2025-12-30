// Generated macro for kv_support (module)
macro_rules! Depcrate___private_apikv_support {
() => {
// Module: crate::__private_api
// Provides: {"kv_support"}
// Dependencies: {}
# [cfg (feature = "kv")] mod kv_support { use crate :: kv ; pub type Value < 'a > = kv :: Value < 'a > ; pub fn capture_to_value < 'a , V : kv :: ToValue + ? Sized > (v : & 'a & 'a V) -> Value < 'a > { v . to_value () } pub fn capture_debug < 'a , V : core :: fmt :: Debug + ? Sized > (v : & 'a & 'a V) -> Value < 'a > { Value :: from_debug (v) } pub fn capture_display < 'a , V : core :: fmt :: Display + ? Sized > (v : & 'a & 'a V) -> Value < 'a > { Value :: from_display (v) } # [cfg (feature = "kv_std")] pub fn capture_error < 'a > (v : & 'a (dyn std :: error :: Error + 'static)) -> Value < 'a > { Value :: from_dyn_error (v) } # [cfg (feature = "kv_sval")] pub fn capture_sval < 'a , V : sval :: Value + ? Sized > (v : & 'a & 'a V) -> Value < 'a > { Value :: from_sval (v) } # [cfg (feature = "kv_serde")] pub fn capture_serde < 'a , V : serde :: Serialize + ? Sized > (v : & 'a & 'a V) -> Value < 'a > { Value :: from_serde (v) } }
};
}
