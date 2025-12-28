macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > Serialize for HistoryBufInner < T , S > where T : Serialize , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self . oldest_ordered () { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_351!()