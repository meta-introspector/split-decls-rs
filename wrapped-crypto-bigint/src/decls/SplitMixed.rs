macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! SplitMixed {
    () => {
        deps!();
        # [doc = " Split a number into parts, returning the least significant part followed by the most"] # [doc = " significant."] pub trait SplitMixed < Lo , Hi > { # [doc = " Split this number into parts, returning its low and high components respectively."] fn split_mixed (& self) -> (Lo , Hi) ; }
    };
}

SplitMixed!();