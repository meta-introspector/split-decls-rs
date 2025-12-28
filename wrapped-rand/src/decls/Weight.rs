macro_rules! deps {
    () => {
        WeightedIndex!();
    };
}

macro_rules! Weight {
    () => {
        deps!();
        # [doc = " Bounds on a weight"] # [doc = ""] # [doc = " See usage in [`WeightedIndex`]."] pub trait Weight : Clone { # [doc = " Representation of 0"] const ZERO : Self ; # [doc = " Checked addition"] # [doc = ""] # [doc = " -   `Result::Ok`: On success, `v` is added to `self`"] # [doc = " -   `Result::Err`: Returns an error when `Self` cannot represent the"] # [doc = "     result of `self + v` (i.e. overflow). The value of `self` should be"] # [doc = "     discarded."] # [allow (clippy :: result_unit_err)] fn checked_add_assign (& mut self , v : & Self) -> Result < () , () > ; }
    };
}

Weight!();