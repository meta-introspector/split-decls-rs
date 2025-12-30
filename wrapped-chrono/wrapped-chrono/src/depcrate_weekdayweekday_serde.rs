// Generated macro for weekday_serde (module)
macro_rules! Depcrate_weekdayweekday_serde {
() => {
// Module: crate::weekday
// Provides: {"weekday_serde"}
// Dependencies: {}
# [cfg (feature = "serde")] mod weekday_serde { use super :: Weekday ; use core :: fmt ; use serde :: { de , ser } ; impl ser :: Serialize for Weekday { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serializer . collect_str (& self) } } struct WeekdayVisitor ; impl de :: Visitor < '_ > for WeekdayVisitor { type Value = Weekday ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("Weekday") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (| _ | E :: custom ("short or long weekday names expected")) } } impl < 'de > de :: Deserialize < 'de > for Weekday { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (WeekdayVisitor) } } }
};
}
