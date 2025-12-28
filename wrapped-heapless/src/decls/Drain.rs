macro_rules! deps {
    () => {
        LenType!();
        Iter!();
        VecView!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator for [`Vec`](super::Vec)."] # [doc = ""] # [doc = " This `struct` is created by [`Vec::drain`](super::Vec::drain)."] # [doc = " See its documentation for more."] pub struct Drain < 'a , T : 'a , LenT : LenType > { # [doc = " Index of tail to preserve"] pub (super) tail_start : LenT , # [doc = " Length of tail"] pub (super) tail_len : LenT , # [doc = " Current remaining range to remove"] pub (super) iter : slice :: Iter < 'a , T > , pub (super) vec : NonNull < VecView < T , LenT > > , }
    };
}

Drain!();