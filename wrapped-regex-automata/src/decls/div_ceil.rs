macro_rules! div_ceil {
    () => {
        # [doc = " Integer division, but rounds up instead of down."] fn div_ceil (lhs : usize , rhs : usize) -> usize { if lhs % rhs == 0 { lhs / rhs } else { (lhs / rhs) + 1 } }
    };
}

div_ceil!();