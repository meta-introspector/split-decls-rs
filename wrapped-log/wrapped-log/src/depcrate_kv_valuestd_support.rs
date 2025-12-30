// Generated macro for std_support (module)
macro_rules! Depcrate_kv_valuestd_support {
() => {
// Module: crate::kv::value
// Provides: {"std_support"}
// Dependencies: {}
# [cfg (feature = "kv_std")] mod std_support { use std :: borrow :: Cow ; use std :: rc :: Rc ; use std :: sync :: Arc ; use super :: * ; impl < T > ToValue for Box < T > where T : ToValue + ? Sized , { fn to_value (& self) -> Value < '_ > { (* * self) . to_value () } } impl < T > ToValue for Arc < T > where T : ToValue + ? Sized , { fn to_value (& self) -> Value < '_ > { (* * self) . to_value () } } impl < T > ToValue for Rc < T > where T : ToValue + ? Sized , { fn to_value (& self) -> Value < '_ > { (* * self) . to_value () } } impl ToValue for String { fn to_value (& self) -> Value < '_ > { Value :: from (& * * self) } } impl < 'v > ToValue for Cow < 'v , str > { fn to_value (& self) -> Value < '_ > { Value :: from (& * * self) } } impl < 'v > Value < 'v > { # [doc = " Try convert this value into a string."] pub fn to_cow_str (& self) -> Option < Cow < 'v , str > > { self . inner . to_str () } } impl < 'v > From < & 'v String > for Value < 'v > { fn from (v : & 'v String) -> Self { Value :: from (& * * v) } } }
};
}
