macro_rules! Square {
    () => {
        # [doc = " Support for optimized squaring"] pub trait Square { # [doc = " Computes the same as `self * self`, but may be more efficient."] fn square (& self) -> Self ; }
    };
}

Square!()