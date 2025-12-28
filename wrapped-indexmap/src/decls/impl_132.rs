macro_rules! deps {
    () => {
        IndexMap!();
        ParValuesMut!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < K , V , S > IndexMap < K , V , S > where K : Send , V : Send , { # [doc = " Return a parallel iterator over mutable references to the values of the map"] # [doc = ""] # [doc = " While parallel iterators can process items in any order, their relative order"] # [doc = " in the map is still preserved for operations like `reduce` and `collect`."] pub fn par_values_mut (& mut self) -> ParValuesMut < '_ , K , V > { ParValuesMut { entries : self . as_entries_mut () , } } }
    };
}

impl_132!();