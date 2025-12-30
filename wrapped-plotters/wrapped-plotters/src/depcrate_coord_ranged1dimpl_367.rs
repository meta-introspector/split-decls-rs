// Generated macro for impl_367 (impl)
macro_rules! Depcrate_coord_ranged1dimpl_367 {
() => {
// Module: crate::coord::ranged1d
// Provides: {"impl_367"}
// Dependencies: {}
impl < R : Ranged < FormatOption = DefaultFormatting > > ValueFormatter < R :: ValueType > for R where R :: ValueType : Debug , { fn format (value : & R :: ValueType) -> String { format ! ("{:?}" , value) } }
};
}
