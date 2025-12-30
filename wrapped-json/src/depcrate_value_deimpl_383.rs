// Generated macro for impl_383 (impl)
macro_rules! Depcrate_value_deimpl_383 {
() => {
// Module: crate::value::de
// Provides: {"impl_383"}
// Dependencies: {}
impl Value { # [cold] fn invalid_type < E > (& self , exp : & dyn Expected) -> E where E : serde :: de :: Error , { serde :: de :: Error :: invalid_type (self . unexpected () , exp) } # [cold] fn unexpected (& self) -> Unexpected { match self { Value :: Null => Unexpected :: Unit , Value :: Bool (b) => Unexpected :: Bool (* b) , Value :: Number (n) => n . unexpected () , Value :: String (s) => Unexpected :: Str (s) , Value :: Array (_) => Unexpected :: Seq , Value :: Object (_) => Unexpected :: Map , } } }
};
}
