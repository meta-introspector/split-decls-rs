macro_rules! SquareAssign {
    () => {
        # [doc = " Support for optimized squaring in-place"] pub trait SquareAssign { # [doc = " Computes the same as `self * self`, but may be more efficient."] # [doc = " Writes the result in `self`."] fn square_assign (& mut self) ; }
    };
}

SquareAssign!();