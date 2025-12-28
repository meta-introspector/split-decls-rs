macro_rules! deps {
    () => {
        ParKeys!();
        ParValues!();
        IndexMap!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [doc = " Parallel iterator methods and other parallel methods."] # [doc = ""] # [doc = " The following methods **require crate feature `\"rayon\"`**."] # [doc = ""] # [doc = " See also the `IntoParallelIterator` implementations."] impl < K , V , S > IndexMap < K , V , S > where K : Sync , V : Sync , { # [doc = " Return a parallel iterator over the keys of the map."] # [doc = ""] # [doc = " While parallel iterators can process items in any order, their relative order"] # [doc = " in the map is still preserved for operations like `reduce` and `collect`."] pub fn par_keys (& self) -> ParKeys < '_ , K , V > { ParKeys { entries : self . as_entries () , } } # [doc = " Return a parallel iterator over the values of the map."] # [doc = ""] # [doc = " While parallel iterators can process items in any order, their relative order"] # [doc = " in the map is still preserved for operations like `reduce` and `collect`."] pub fn par_values (& self) -> ParValues < '_ , K , V > { ParValues { entries : self . as_entries () , } } }
    };
}

impl_119!();