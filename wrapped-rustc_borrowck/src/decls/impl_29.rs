macro_rules! deps {
    () => {
        EdgesFromStatic!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Iterator for EdgesFromStatic { type Item = RegionVid ; fn next (& mut self) -> Option < Self :: Item > { if self . next_static_idx < self . end_static_idx { let ret = RegionVid :: from_usize (self . next_static_idx) ; self . next_static_idx += 1 ; Some (ret) } else { None } } }
    };
}

impl_29!()