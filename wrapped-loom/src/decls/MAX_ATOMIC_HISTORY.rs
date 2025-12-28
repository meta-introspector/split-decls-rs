macro_rules! MAX_ATOMIC_HISTORY {
    () => {
        # [doc = " Maximum number of atomic store history to track per-cell."] pub (crate) const MAX_ATOMIC_HISTORY : usize = 7 ;
    };
}

MAX_ATOMIC_HISTORY!();