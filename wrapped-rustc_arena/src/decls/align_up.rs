macro_rules! align_up {
    () => {
        # [inline (always)] fn align_up (val : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; (val + align - 1) & ! (align - 1) }
    };
}

align_up!();