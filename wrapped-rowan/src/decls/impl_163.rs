macro_rules! deps {
    () => {
        SerDisplay!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < T : fmt :: Display > Serialize for SerDisplay < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (& self . 0) } }
    };
}

impl_163!()