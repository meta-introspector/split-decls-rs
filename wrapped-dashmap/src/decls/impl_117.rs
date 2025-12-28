macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < K : Eq + Hash , S : BuildHasher + Clone > PartialEq for DashSet < K , S > { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . all (| r | other . contains (r . key ())) } }
    };
}

impl_117!();