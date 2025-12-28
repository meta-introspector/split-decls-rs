macro_rules! deps {
    () => {
        DenseBitSet!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > Clone for DenseBitSet < T > { fn clone (& self) -> Self { DenseBitSet { domain_size : self . domain_size , words : self . words . clone () , marker : PhantomData , } } fn clone_from (& mut self , from : & Self) { self . domain_size = from . domain_size ; self . words . clone_from (& from . words) ; } }
    };
}

impl_16!()