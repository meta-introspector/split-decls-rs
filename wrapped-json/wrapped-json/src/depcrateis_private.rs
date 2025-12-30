// Generated macro for is_private (function)
macro_rules! Depcrateis_private {
() => {
// Module: crate
// Provides: {"is_private"}
// Dependencies: {}
# [cfg (feature = "serde")] fn is_private (data : & Data) -> bool { match data { Data :: Private => true , Data :: Struct (_) | Data :: Enum (_) => false , } }
};
}
