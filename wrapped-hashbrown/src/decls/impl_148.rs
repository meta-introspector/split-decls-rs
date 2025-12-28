macro_rules! deps {
    () => {
        ParKeys!();
        ParValues!();
        HashMap!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < K : Sync , V : Sync , S , A : Allocator > HashMap < K , V , S , A > { # [doc = " Visits (potentially in parallel) immutably borrowed keys in an arbitrary order."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_keys (& self) -> ParKeys < '_ , K , V > { ParKeys { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } # [doc = " Visits (potentially in parallel) immutably borrowed values in an arbitrary order."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_values (& self) -> ParValues < '_ , K , V > { ParValues { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } }
    };
}

impl_148!()