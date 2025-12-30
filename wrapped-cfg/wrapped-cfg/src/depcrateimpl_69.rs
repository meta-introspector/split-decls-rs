// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl fmt :: Display for CfgDiff { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ! self . enable . is_empty () { f . write_str ("enable ") ? ; for (i , atom) in self . enable . iter () . enumerate () { let sep = match i { 0 => "" , _ if i == self . enable . len () - 1 => " and " , _ => ", " , } ; f . write_str (sep) ? ; atom . fmt (f) ? ; } if ! self . disable . is_empty () { f . write_str ("; ") ? ; } } if ! self . disable . is_empty () { f . write_str ("disable ") ? ; for (i , atom) in self . disable . iter () . enumerate () { let sep = match i { 0 => "" , _ if i == self . enable . len () - 1 => " and " , _ => ", " , } ; f . write_str (sep) ? ; atom . fmt (f) ? ; } } Ok (()) } }
};
}
