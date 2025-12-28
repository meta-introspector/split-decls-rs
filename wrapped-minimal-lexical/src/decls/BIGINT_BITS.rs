macro_rules! deps {
    () => {
        Number!();
        Bigint!();
    };
}

macro_rules! BIGINT_BITS {
    () => {
        deps!();
        # [doc = " Number of bits in a Bigint."] # [doc = ""] # [doc = " This needs to be at least the number of bits required to store"] # [doc = " a Bigint, which is `log2(radix**digits)`."] # [doc = " ≅ 3600 for base-10, rounded-up."] pub const BIGINT_BITS : usize = 4000 ;
    };
}

BIGINT_BITS!();