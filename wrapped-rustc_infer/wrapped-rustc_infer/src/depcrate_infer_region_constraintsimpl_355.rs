// Generated macro for impl_355 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_355 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_355"}
// Dependencies: {}
impl < 'tcx > VerifyBound < 'tcx > { pub fn must_hold (& self) -> bool { match self { VerifyBound :: IfEq (..) => false , VerifyBound :: OutlivedBy (re) => re . is_static () , VerifyBound :: IsEmpty => false , VerifyBound :: AnyBound (bs) => bs . iter () . any (| b | b . must_hold ()) , VerifyBound :: AllBounds (bs) => bs . iter () . all (| b | b . must_hold ()) , } } pub fn cannot_hold (& self) -> bool { match self { VerifyBound :: IfEq (..) => false , VerifyBound :: IsEmpty => false , VerifyBound :: OutlivedBy (_) => false , VerifyBound :: AnyBound (bs) => bs . iter () . all (| b | b . cannot_hold ()) , VerifyBound :: AllBounds (bs) => bs . iter () . any (| b | b . cannot_hold ()) , } } pub fn or (self , vb : VerifyBound < 'tcx >) -> VerifyBound < 'tcx > { if self . must_hold () || vb . cannot_hold () { self } else if self . cannot_hold () || vb . must_hold () { vb } else { VerifyBound :: AnyBound (vec ! [self , vb]) } } }
};
}
