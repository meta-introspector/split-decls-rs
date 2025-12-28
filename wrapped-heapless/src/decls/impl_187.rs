macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < K , V1 , V2 , S1 : LinearMapStorage < K , V1 > + ? Sized , S2 : LinearMapStorage < K , V2 > + ? Sized > PartialEq < LinearMapInner < K , V2 , S2 > > for LinearMapInner < K , V1 , S1 > where K : Eq , V1 : PartialEq < V2 > , { fn eq (& self , other : & LinearMapInner < K , V2 , S2 >) -> bool { self . len () == other . len () && self . iter () . all (| (key , value) | other . get (key) . is_some_and (| v | * value == * v)) } }
    };
}

impl_187!();