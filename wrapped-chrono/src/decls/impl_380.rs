macro_rules! deps {
    () => {
        Error!();
        NaiveDateTime!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        # [doc = " Serialize a `NaiveDateTime` as an ISO 8601 string"] # [doc = ""] # [doc = " See [the `naive::serde` module](crate::naive::serde) for alternate serialization formats."] impl ser :: Serialize for NaiveDateTime { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { struct FormatWrapped < 'a , D : 'a > { inner : & 'a D , } impl < D : fmt :: Debug > fmt :: Display for FormatWrapped < '_ , D > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . inner . fmt (f) } } serializer . collect_str (& FormatWrapped { inner : & self }) } }
    };
}

impl_380!()