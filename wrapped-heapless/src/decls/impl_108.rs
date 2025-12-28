macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < K , V1 , V2 , S1 , S2 , const N1 : usize , const N2 : usize > PartialEq < IndexMap < K , V2 , S2 , N2 > > for IndexMap < K , V1 , S1 , N1 > where K : Eq + Hash , V1 : PartialEq < V2 > , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexMap < K , V2 , S2 , N2 >) -> bool { self . len () == other . len () && self . iter () . all (| (key , value) | other . get (key) . is_some_and (| v | * value == * v)) } }
    };
}

impl_108!()