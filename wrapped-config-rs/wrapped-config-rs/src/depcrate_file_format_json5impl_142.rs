// Generated macro for impl_142 (impl)
macro_rules! Depcrate_file_format_json5impl_142 {
() => {
// Module: crate::file::format::json5
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'de > serde_core :: de :: Deserialize < 'de > for Val { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { serde_untagged :: UntaggedEnumVisitor :: new () . bool (| value | Ok (Self :: Boolean (value))) . i64 (| value | Ok (Self :: Integer (value))) . f64 (| value | Ok (Self :: Float (value))) . string (| value | Ok (Val :: String (value . to_owned ()))) . unit (| | Ok (Self :: Null)) . seq (| value | value . deserialize () . map (Val :: Array)) . map (| value | value . deserialize () . map (Val :: Object)) . deserialize (d) } }
};
}
