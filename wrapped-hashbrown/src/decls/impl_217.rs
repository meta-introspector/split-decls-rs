macro_rules! deps {
    () => {
        ParDrain!();
        HashTable!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < T : Send , A : Allocator > HashTable < T , A > { # [doc = " Consumes (potentially in parallel) all values in an arbitrary order,"] # [doc = " while preserving the map's allocated memory for reuse."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_drain (& mut self) -> ParDrain < '_ , T , A > { ParDrain { inner : self . raw . par_drain () , } } }
    };
}

impl_217!()