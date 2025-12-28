macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < K , V , S : LinearMapStorage < K , V > + ? Sized > Serialize for LinearMapInner < K , V , S > where K : Eq + Serialize , V : Serialize , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map . serialize_entry (k , v) ? ; } map . end () } }
    };
}

impl_353!();