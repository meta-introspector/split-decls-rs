// Generated macro for impl_130 (impl)
macro_rules! Depcrate_binary16impl_130 {
() => {
// Module: crate::binary16
// Provides: {"impl_130"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: de :: Visitor < 'de > for Visitor { type Value = f16 ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (formatter , "tuple struct f16") } fn visit_newtype_struct < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: Deserializer < 'de > , { Ok (f16 (< u16 as Deserialize > :: deserialize (deserializer) ?)) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { v . parse () . map_err (| _ | { serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (v) , & "a float string") }) } fn visit_f32 < E > (self , v : f32) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (f16 :: from_f32 (v)) } fn visit_f64 < E > (self , v : f64) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (f16 :: from_f64 (v)) } }
};
}
