// Generated macro for impl_18 (impl)
macro_rules! Depcrate_attrimpl_18 {
() => {
// Module: crate::attr
// Provides: {"impl_18"}
// Dependencies: {}
impl IndexAttr { fn const_from_lit (& self , lit : & Lit) -> isize { if let Lit :: Int (ref n) = lit { n . base10_parse () . expect ("invalid value") } else { panic ! ("unexpected value") } } }
};
}
