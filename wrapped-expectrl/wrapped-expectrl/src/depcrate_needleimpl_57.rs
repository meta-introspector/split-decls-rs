// Generated macro for impl_57 (impl)
macro_rules! Depcrate_needleimpl_57 {
() => {
// Module: crate::needle
// Provides: {"impl_57"}
// Dependencies: {}
impl Needle for [u8] { fn check (& self , buf : & [u8] , _ : bool) -> Result < Vec < Match > , Error > { if buf . len () < self . len () { return Ok (Vec :: new ()) ; } for l_bound in 0 .. buf . len () { let r_bound = l_bound + self . len () ; if r_bound > buf . len () { return Ok (Vec :: new ()) ; } if self == & buf [l_bound .. r_bound] { return Ok (vec ! [Match :: new (l_bound , r_bound)]) ; } } Ok (Vec :: new ()) } }
};
}
