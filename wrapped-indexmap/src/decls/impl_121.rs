macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < K , V , S > IndexMap < K , V , S > where K : Hash + Eq + Sync , V : Sync , S : BuildHasher , { # [doc = " Returns `true` if `self` contains all of the same key-value pairs as `other`,"] # [doc = " regardless of each map's indexed order, determined in parallel."] pub fn par_eq < V2 , S2 > (& self , other : & IndexMap < K , V2 , S2 >) -> bool where V : PartialEq < V2 > , V2 : Sync , S2 : BuildHasher + Sync , { self . len () == other . len () && self . par_iter () . all (move | (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
    };
}

impl_121!();