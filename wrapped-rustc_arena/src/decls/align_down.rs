macro_rules! align_down {
    () => {
        # [inline (always)] fn align_down (val : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; val & ! (align - 1) }
    };
}

align_down!();