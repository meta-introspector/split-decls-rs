macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for RawTable < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . table . drop_inner_table :: < T , _ > (& self . alloc , Self :: TABLE_LAYOUT) ; } } }
    };
}

impl_65!();