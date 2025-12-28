macro_rules! deps {
    () => {
        NaiveTime!();
        Error!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl ser :: Serialize for NaiveTime { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serializer . collect_str (& self) } }
    };
}

impl_483!()