macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T , S > Serialize for IndexSet < T , S > where T : Serialize , { fn serialize < Se > (& self , serializer : Se) -> Result < Se :: Ok , Se :: Error > where Se : Serializer , { serializer . collect_seq (self) } }
    };
}

impl_27!()