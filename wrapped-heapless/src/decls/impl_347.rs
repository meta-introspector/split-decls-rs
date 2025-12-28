macro_rules! deps {
    () => {
        BinaryHeapInner!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < T , KIND , S > Serialize for BinaryHeapInner < T , KIND , S > where T : Ord + Serialize , KIND : BinaryHeapKind , S : VecStorage < T > + ? Sized , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_347!()