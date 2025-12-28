macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! Kernel {
    () => {
        deps!();
        # [doc = " Kernel function"] pub trait Kernel < A > : Copy + Sync where A : Float , { # [doc = " Apply the kernel function to the given x-value."] fn evaluate (& self , x : A) -> A ; }
    };
}

Kernel!();