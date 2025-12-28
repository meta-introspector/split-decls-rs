macro_rules! deps {
    () => {
        ChunkedBitSet!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > Clone for ChunkedBitSet < T > { fn clone (& self) -> Self { ChunkedBitSet { domain_size : self . domain_size , chunks : self . chunks . clone () , marker : PhantomData , } } # [doc = " WARNING: this implementation of clone_from will panic if the two"] # [doc = " bitsets have different domain sizes. This constraint is not inherent to"] # [doc = " `clone_from`, but it works with the existing call sites and allows a"] # [doc = " faster implementation, which is important because this function is hot."] fn clone_from (& mut self , from : & Self) { assert_eq ! (self . domain_size , from . domain_size) ; debug_assert_eq ! (self . chunks . len () , from . chunks . len ()) ; self . chunks . clone_from (& from . chunks) } }
    };
}

impl_29!();