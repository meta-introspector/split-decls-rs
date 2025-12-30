// Generated macro for do_insert_phase_two (function)
macro_rules! Depcrate_header_mapdo_insert_phase_two {
() => {
// Module: crate::header::map
// Provides: {"do_insert_phase_two"}
// Dependencies: {}
# [doc = " phase 2 is post-insert where we forward-shift `Pos` in the indices."] # [doc = ""] # [doc = " returns the number of displaced elements"] # [inline] fn do_insert_phase_two (indices : & mut [Pos] , mut probe : usize , mut old_pos : Pos) -> usize { let mut num_displaced = 0 ; probe_loop ! (probe < indices . len () , { let pos = & mut indices [probe] ; if pos . is_none () { * pos = old_pos ; break ; } else { num_displaced += 1 ; old_pos = mem :: replace (pos , old_pos) ; } }) ; num_displaced }
};
}
