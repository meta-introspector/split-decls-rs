macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < K , V1 , S1 , V2 , S2 > PartialEq < IndexMap < K , V2 , S2 > > for IndexMap < K , V1 , S1 > where K : Hash + Eq , V1 : PartialEq < V2 > , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexMap < K , V2 , S2 >) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
    };
}

impl_65!();