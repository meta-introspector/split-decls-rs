macro_rules! deps {
    () => {
        Blake2b!();
    };
}

macro_rules! Hasher {
    () => {
        deps!();
        # [allow (clippy :: derive_partial_eq_without_eq)] # [derive (Debug , PartialEq)] # [doc = " Convenience functions for common BLAKE2b operations."] pub enum Hasher { # [doc = " Blake2b with `32` as `size`."] Blake2b256 , # [doc = " Blake2b with `48` as `size`."] Blake2b384 , # [doc = " Blake2b with `64` as `size`."] Blake2b512 , }
    };
}

Hasher!();