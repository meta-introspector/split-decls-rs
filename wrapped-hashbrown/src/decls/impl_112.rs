macro_rules! deps {
    () => {
        RawIterHashInner!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Iterator for RawIterHashInner { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { unsafe { loop { if let Some (bit) = self . bitmask . next () { let index = (self . probe_seq . pos + bit) & self . bucket_mask ; return Some (index) ; } if likely (self . group . match_empty () . any_bit_set ()) { return None ; } self . probe_seq . move_next (self . bucket_mask) ; let index = self . probe_seq . pos ; debug_assert ! (index < self . bucket_mask + 1 + Group :: WIDTH) ; let group_ctrl = self . ctrl . as_ptr () . add (index) . cast () ; self . group = Group :: load (group_ctrl) ; self . bitmask = self . group . match_tag (self . tag_hash) . into_iter () ; } } } }
    };
}

impl_112!();