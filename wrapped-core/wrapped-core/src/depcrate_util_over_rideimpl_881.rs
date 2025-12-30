// Generated macro for impl_881 (impl)
macro_rules! Depcrate_util_over_rideimpl_881 {
() => {
// Module: crate::util::over_ride
// Provides: {"impl_881"}
// Dependencies: {}
# [doc = " Parses a `Meta`. A bare word will produce `Override::Inherit`, while"] # [doc = " any value will be forwarded to `T::from_meta`."] impl < T : FromMeta > FromMeta for Override < T > { fn from_meta (item : & syn :: Meta) -> Result < Self > { match item { syn :: Meta :: Path (_) => Self :: from_word () , _ => FromMeta :: from_meta (item) . map (Explicit) , } } fn from_word () -> Result < Self > { Ok (Inherit) } fn from_list (items : & [NestedMeta]) -> Result < Self > { Ok (Explicit (FromMeta :: from_list (items) ?)) } fn from_value (lit : & Lit) -> Result < Self > { Ok (Explicit (FromMeta :: from_value (lit) ?)) } fn from_expr (expr : & syn :: Expr) -> Result < Self > { Ok (Explicit (FromMeta :: from_expr (expr) ?)) } fn from_char (value : char) -> Result < Self > { Ok (Explicit (FromMeta :: from_char (value) ?)) } fn from_string (value : & str) -> Result < Self > { Ok (Explicit (FromMeta :: from_string (value) ?)) } fn from_bool (value : bool) -> Result < Self > { Ok (Explicit (FromMeta :: from_bool (value) ?)) } }
};
}
