macro_rules! deps {
    () => {
        HashMap!();
        ParValuesMut!();
        ParDrain!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < K : Send , V : Send , S , A : Allocator > HashMap < K , V , S , A > { # [doc = " Visits (potentially in parallel) mutably borrowed values in an arbitrary order."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_values_mut (& mut self) -> ParValuesMut < '_ , K , V > { ParValuesMut { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } # [doc = " Consumes (potentially in parallel) all values in an arbitrary order,"] # [doc = " while preserving the map's allocated memory for reuse."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_drain (& mut self) -> ParDrain < '_ , K , V , A > { ParDrain { inner : self . table . par_drain () , } } }
    };
}

impl_149!();