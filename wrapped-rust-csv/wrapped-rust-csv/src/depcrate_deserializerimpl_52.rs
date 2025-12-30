// Generated macro for impl_52 (impl)
macro_rules! Depcrate_deserializerimpl_52 {
() => {
// Module: crate::deserializer
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'r , T : DeRecord < 'r > > DeRecord < 'r > for DeRecordWrap < T > { # [inline] fn has_headers (& self) -> bool { self . 0 . has_headers () } # [inline] fn next_header (& mut self) -> Result < Option < & 'r str > , DeserializeError > { self . 0 . next_header () } # [inline] fn next_header_bytes (& mut self ,) -> Result < Option < & 'r [u8] > , DeserializeError > { self . 0 . next_header_bytes () } # [inline] fn next_field (& mut self) -> Result < & 'r str , DeserializeError > { self . 0 . next_field () } # [inline] fn next_field_bytes (& mut self) -> Result < & 'r [u8] , DeserializeError > { self . 0 . next_field_bytes () } # [inline] fn peek_field (& mut self) -> Option < & 'r [u8] > { self . 0 . peek_field () } # [inline] fn error (& self , kind : DeserializeErrorKind) -> DeserializeError { self . 0 . error (kind) } # [inline] fn infer_deserialize < 'de , V : Visitor < 'de > > (& mut self , visitor : V ,) -> Result < V :: Value , DeserializeError > { self . 0 . infer_deserialize (visitor) } }
};
}
