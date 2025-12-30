// Generated macro for copy_nonoverlapping_small (function)
macro_rules! Depcrate_sip128copy_nonoverlapping_small {
() => {
// Module: crate::sip128
// Provides: {"copy_nonoverlapping_small"}
// Dependencies: {}
# [inline] unsafe fn copy_nonoverlapping_small (src : * const u8 , dst : * mut u8 , count : usize) { debug_assert ! (count <= 8) ; unsafe { if count == 8 { ptr :: copy_nonoverlapping (src , dst , 8) ; return ; } let mut i = 0 ; if i . debug_strict_add (3) < count { ptr :: copy_nonoverlapping (src . add (i) , dst . add (i) , 4) ; i = i . debug_strict_add (4) ; } if i . debug_strict_add (1) < count { ptr :: copy_nonoverlapping (src . add (i) , dst . add (i) , 2) ; i = i . debug_strict_add (2) } if i < count { * dst . add (i) = * src . add (i) ; i = i . debug_strict_add (1) ; } debug_assert_eq ! (i , count) ; } }
};
}
