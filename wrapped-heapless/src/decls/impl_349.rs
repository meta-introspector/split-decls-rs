macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < T , LenT : LenType , St : VecStorage < T > > Serialize for VecInner < T , LenT , St > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_349!()