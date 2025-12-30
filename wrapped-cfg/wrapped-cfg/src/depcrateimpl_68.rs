// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl CfgDiff { # [doc = " Create a new CfgDiff."] pub fn new (mut enable : Vec < CfgAtom > , mut disable : Vec < CfgAtom >) -> CfgDiff { enable . sort () ; enable . dedup () ; disable . sort () ; disable . dedup () ; for i in (0 .. enable . len ()) . rev () { if let Some (j) = disable . iter () . position (| atom | * atom == enable [i]) { enable . remove (i) ; disable . remove (j) ; } } CfgDiff { enable , disable } } # [doc = " Returns the total number of atoms changed by this diff."] pub fn len (& self) -> usize { self . enable . len () + self . disable . len () } pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
