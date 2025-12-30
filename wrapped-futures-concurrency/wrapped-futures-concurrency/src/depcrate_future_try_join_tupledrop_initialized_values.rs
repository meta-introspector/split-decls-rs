// Generated macro for drop_initialized_values (macro)
macro_rules! Depcrate_future_try_join_tupledrop_initialized_values {
() => {
// Module: crate::future::try_join::tuple
// Provides: {"drop_initialized_values"}
// Dependencies: {}
# [doc = " Drop all initialized values"] macro_rules ! drop_initialized_values { (@ drop $ output : ident , $ ($ rem_outs : ident ,) * | $ states : expr , $ state_idx : tt , $ ($ rem_idx : tt ,) *) => { if $ states [$ state_idx] . is_ready () { unsafe { $ output . assume_init_drop () } ; $ states [$ state_idx] . set_none () ; } drop_initialized_values ! (@ drop $ ($ rem_outs ,) * | $ states , $ ($ rem_idx ,) *) ; } ; (@ drop | $ states : expr , $ ($ rem_idx : tt ,) *) => { } ; ($ ($ outs : ident ,) + | $ states : expr) => { drop_initialized_values ! (@ drop $ ($ outs ,) + | $ states , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 ,) ; } ; }
};
}
