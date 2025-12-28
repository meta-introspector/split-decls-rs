macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < T , S : VecStorage < T > + ? Sized > Serialize for DequeInner < T , S > where T : Serialize , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . storage_len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_350!()