// Generated macro for impl_2180 (impl)
macro_rules! Depcrate_float_literalimpl_2180 {
() => {
// Module: crate::float_literal
// Provides: {"impl_2180"}
// Dependencies: {}
impl FloatFormat { # [must_use] fn new (s : & str) -> Self { s . chars () . find_map (| x | match x { 'e' => Some (Self :: LowerExp) , 'E' => Some (Self :: UpperExp) , _ => None , }) . unwrap_or (Self :: Normal) } fn format < T > (& self , f : T) -> String where T : fmt :: UpperExp + fmt :: LowerExp + fmt :: Display , { match self { Self :: LowerExp => format ! ("{f:e}") , Self :: UpperExp => format ! ("{f:E}") , Self :: Normal => format ! ("{f}") , } } }
};
}
