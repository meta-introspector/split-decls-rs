// Generated macro for ValueFormatter (trait)
macro_rules! Depcrate_coord_ranged1dValueFormatter {
() => {
// Module: crate::coord::ranged1d
// Provides: {"ValueFormatter"}
// Dependencies: {}
# [doc = " Determine how we can format a value in a coordinate system by default"] pub trait ValueFormatter < V > { # [doc = " Format the value"] fn format (_value : & V) -> String { panic ! ("Unimplemented formatting method") ; } # [doc = " Determine how we can format a value in a coordinate system by default"] fn format_ext (& self , value : & V) -> String { Self :: format (value) } }
};
}
