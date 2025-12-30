// Generated macro for impl_65 (impl)
macro_rules! Depcrate_bfloatimpl_65 {
() => {
// Module: crate::bfloat
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: de :: Visitor < 'de > for Visitor { type Value = bf16 ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (formatter , "tuple struct bf16") } fn visit_newtype_struct < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: Deserializer < 'de > , { Ok (bf16 (< u16 as Deserialize > :: deserialize (deserializer) ?)) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { v . parse () . map_err (| _ | { serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (v) , & "a float string") }) } fn visit_f32 < E > (self , v : f32) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (bf16 :: from_f32 (v)) } fn visit_f64 < E > (self , v : f64) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (bf16 :: from_f64 (v)) } }
};
}
