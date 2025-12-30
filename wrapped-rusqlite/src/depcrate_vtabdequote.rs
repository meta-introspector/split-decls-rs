// Generated macro for dequote (function)
macro_rules! Depcrate_vtabdequote {
() => {
// Module: crate::vtab
// Provides: {"dequote"}
// Dependencies: {}
# [doc = " Dequote string"] # [must_use] pub fn dequote (s : & str) -> & str { if s . len () < 2 { return s ; } match s . bytes () . next () { Some (b) if b == b'"' || b == b'\'' => match s . bytes () . next_back () { Some (e) if e == b => & s [1 .. s . len () - 1] , _ => s , } , _ => s , } }
};
}
