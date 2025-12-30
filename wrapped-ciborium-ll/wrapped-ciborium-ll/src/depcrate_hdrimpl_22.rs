// Generated macro for impl_22 (impl)
macro_rules! Depcrate_hdrimpl_22 {
() => {
// Module: crate::hdr
// Provides: {"impl_22"}
// Dependencies: {}
impl TryFrom < Title > for Header { type Error = InvalidError ; # [inline] fn try_from (title : Title) -> Result < Self , Self :: Error > { let opt = | minor | { Some (match minor { Minor :: This (x) => x . into () , Minor :: Next1 (x) => u8 :: from_be_bytes (x) . into () , Minor :: Next2 (x) => u16 :: from_be_bytes (x) . into () , Minor :: Next4 (x) => u32 :: from_be_bytes (x) . into () , Minor :: Next8 (x) => u64 :: from_be_bytes (x) , Minor :: More => return None , }) } ; let int = | m | opt (m) . ok_or (InvalidError (())) ; let len = | m | { opt (m) . map (usize :: try_from) . transpose () . or (Err (InvalidError (()))) } ; Ok (match title { Title (Major :: Positive , minor) => Self :: Positive (int (minor) ?) , Title (Major :: Negative , minor) => Self :: Negative (int (minor) ?) , Title (Major :: Bytes , minor) => Self :: Bytes (len (minor) ?) , Title (Major :: Text , minor) => Self :: Text (len (minor) ?) , Title (Major :: Array , minor) => Self :: Array (len (minor) ?) , Title (Major :: Map , minor) => Self :: Map (len (minor) ?) , Title (Major :: Tag , minor) => Self :: Tag (int (minor) ?) , Title (Major :: Other , Minor :: More) => Self :: Break , Title (Major :: Other , Minor :: This (x)) => Self :: Simple (x) , Title (Major :: Other , Minor :: Next1 (x)) => Self :: Simple (x [0]) , Title (Major :: Other , Minor :: Next2 (x)) => Self :: Float (f16 :: from_be_bytes (x) . into ()) , Title (Major :: Other , Minor :: Next4 (x)) => Self :: Float (f32 :: from_be_bytes (x) . into ()) , Title (Major :: Other , Minor :: Next8 (x)) => Self :: Float (f64 :: from_be_bytes (x)) , }) } }
};
}
