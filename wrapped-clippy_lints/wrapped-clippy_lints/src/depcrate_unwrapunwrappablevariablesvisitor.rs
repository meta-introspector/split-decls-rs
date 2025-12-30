// Generated macro for UnwrappableVariablesVisitor (struct)
macro_rules! Depcrate_unwrapUnwrappableVariablesVisitor {
() => {
// Module: crate::unwrap
// Provides: {"UnwrappableVariablesVisitor"}
// Dependencies: {}
# [doc = " Visitor that keeps track of which variables are unwrappable."] struct UnwrappableVariablesVisitor < 'a , 'tcx > { unwrappables : Vec < UnwrapInfo < 'tcx > > , cx : & 'a LateContext < 'tcx > , msrv : Msrv , }
};
}
