// Generated macro for RcInnerPtr (trait)
macro_rules! Depcrate_rcRcInnerPtr {
() => {
// Module: crate::rc
// Provides: {"RcInnerPtr"}
// Dependencies: {}
# [doc (hidden)] trait RcInnerPtr { fn weak_ref (& self) -> & Cell < usize > ; fn strong_ref (& self) -> & Cell < usize > ; # [inline] fn strong (& self) -> usize { self . strong_ref () . get () } # [inline] fn inc_strong (& self) { let strong = self . strong () ; unsafe { hint :: assert_unchecked (strong != 0) ; } let strong = strong . wrapping_add (1) ; self . strong_ref () . set (strong) ; if core :: intrinsics :: unlikely (strong == 0) { abort () ; } } # [inline] fn dec_strong (& self) { self . strong_ref () . set (self . strong () - 1) ; } # [inline] fn weak (& self) -> usize { self . weak_ref () . get () } # [inline] fn inc_weak (& self) { let weak = self . weak () ; unsafe { hint :: assert_unchecked (weak != 0) ; } let weak = weak . wrapping_add (1) ; self . weak_ref () . set (weak) ; if core :: intrinsics :: unlikely (weak == 0) { abort () ; } } # [inline] fn dec_weak (& self) { self . weak_ref () . set (self . weak () - 1) ; } }
};
}
