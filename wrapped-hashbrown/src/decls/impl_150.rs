macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < K , V , S , A > HashMap < K , V , S , A > where K : Eq + Hash + Sync , V : PartialEq + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { # [doc = " Returns `true` if the map is equal to another,"] # [doc = " i.e. both maps contain the same keys mapped to the same values."] # [doc = ""] # [doc = " This method runs in a potentially parallel fashion."] pub fn par_eq (& self , other : & Self) -> bool { self . len () == other . len () && self . into_par_iter () . all (| (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
    };
}

impl_150!();