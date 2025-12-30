// Generated macro for is_midpoint_implemented (function)
macro_rules! Depcrate_operators_manual_midpointis_midpoint_implemented {
() => {
// Module: crate::operators::manual_midpoint
// Provides: {"is_midpoint_implemented"}
// Dependencies: {}
fn is_midpoint_implemented (cx : & LateContext < '_ > , ty : Ty < '_ > , msrv : Msrv) -> bool { match ty . kind () { ty :: Uint (_) | ty :: Float (_) => msrv . meets (cx , msrvs :: UINT_FLOAT_MIDPOINT) , ty :: Int (_) => msrv . meets (cx , msrvs :: INT_MIDPOINT) , _ => false , } }
};
}
