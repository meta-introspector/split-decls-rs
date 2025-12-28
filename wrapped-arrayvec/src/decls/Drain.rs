macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator for `ArrayVec`."] pub struct Drain < 'a , T : 'a , const CAP : usize > { # [doc = " Index of tail to preserve"] tail_start : usize , # [doc = " Length of tail"] tail_len : usize , # [doc = " Current remaining range to remove"] iter : slice :: Iter < 'a , T > , vec : * mut ArrayVec < T , CAP > , }
    };
}

Drain!();