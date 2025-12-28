macro_rules! deps {
    () => {
        Uniform!();
    };
}

macro_rules! IncreasingUniform {
    () => {
        deps!();
        # [doc = " Similar to a Uniform distribution,"] # [doc = " but after returning a number in the range [0,n], n is increased by 1."] pub (crate) struct IncreasingUniform < R : RngCore > { pub rng : R , n : u32 , chunk : u32 , chunk_remaining : u8 , }
    };
}

IncreasingUniform!()