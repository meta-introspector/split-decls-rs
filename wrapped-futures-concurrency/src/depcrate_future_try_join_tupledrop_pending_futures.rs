// Generated macro for drop_pending_futures (macro)
macro_rules! Depcrate_future_try_join_tupledrop_pending_futures {
() => {
// Module: crate::future::try_join::tuple
// Provides: {"drop_pending_futures"}
// Dependencies: {}
# [doc = " Drop all pending futures"] macro_rules ! drop_pending_futures { (@ inner $ states : ident , $ futures : ident , $ fut_name : ident $ ($ F : ident) * | $ fut_idx : tt $ ($ rest : tt) *) => { if $ states [$ fut_idx] . is_pending () { let futures = unsafe { $ futures . as_mut () . get_unchecked_mut () } ; unsafe { ManuallyDrop :: drop (& mut futures .$ fut_name) } ; } drop_pending_futures ! (@ inner $ states , $ futures , $ ($ F) * | $ ($ rest) *) ; } ; (@ inner $ states : ident , $ futures : ident , | $ ($ rest : tt) *) => { } ; ($ states : ident , $ futures : ident , $ ($ F : ident ,) +) => { drop_pending_futures ! (@ inner $ states , $ futures , $ ($ F) + | 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14) ; } ; }
};
}
