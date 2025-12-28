macro_rules! deps {
    () => {
        Error!();
        Month!();
    };
}

macro_rules! month_serde {
    () => {
        deps!();
        # [cfg (feature = "serde")] mod month_serde { use super :: Month ; use serde :: { de , ser } ; use core :: fmt ; impl ser :: Serialize for Month { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serializer . collect_str (self . name ()) } } struct MonthVisitor ; impl de :: Visitor < '_ > for MonthVisitor { type Value = Month ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("Month") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (| _ | E :: custom ("short (3-letter) or full month names expected")) } } impl < 'de > de :: Deserialize < 'de > for Month { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (MonthVisitor) } } }
    };
}

month_serde!();