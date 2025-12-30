// Generated macro for atomic (macro)
macro_rules! Depcrate_atomic_atomic_cellatomic {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"atomic"}
// Dependencies: {}
macro_rules ! atomic { (@ check , $ t : ty , $ atomic : ty , $ a : ident , $ atomic_op : expr) => { if can_transmute ::<$ t , $ atomic > () { let $ a : &$ atomic ; break $ atomic_op ; } } ; ($ t : ty , $ a : ident , $ atomic_op : expr , $ fallback_op : expr) => { loop { atomic ! (@ check , $ t , AtomicUnit , $ a , $ atomic_op) ; # [cfg (not (any (miri , crossbeam_loom , crossbeam_atomic_cell_force_fallback ,)))] atomic_maybe_uninit :: cfg_has_atomic_cas ! { atomic_maybe_uninit :: cfg_has_atomic_8 ! { atomic ! (@ check , $ t , atomic_maybe_uninit :: AtomicMaybeUninit < u8 >, $ a , $ atomic_op) ; } atomic_maybe_uninit :: cfg_has_atomic_16 ! { atomic ! (@ check , $ t , atomic_maybe_uninit :: AtomicMaybeUninit < u16 >, $ a , $ atomic_op) ; } atomic_maybe_uninit :: cfg_has_atomic_32 ! { atomic ! (@ check , $ t , atomic_maybe_uninit :: AtomicMaybeUninit < u32 >, $ a , $ atomic_op) ; } atomic_maybe_uninit :: cfg_has_atomic_64 ! { atomic ! (@ check , $ t , atomic_maybe_uninit :: AtomicMaybeUninit < u64 >, $ a , $ atomic_op) ; } atomic_maybe_uninit :: cfg_has_atomic_128 ! { atomic ! (@ check , $ t , atomic_maybe_uninit :: AtomicMaybeUninit < u128 >, $ a , $ atomic_op) ; } } break $ fallback_op ; } } ; }
};
}
