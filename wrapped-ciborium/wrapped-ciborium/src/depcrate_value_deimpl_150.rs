// Generated macro for impl_150 (impl)
macro_rules! Depcrate_value_deimpl_150 {
() => {
// Module: crate::value::de
// Provides: {"impl_150"}
// Dependencies: {}
impl Deserializer < & Value > { fn integer < N > (& self , kind : & 'static str) -> Result < N , Error > where N : TryFrom < u128 > , N : TryFrom < i128 > , { fn raw (value : & Value) -> Result < u128 , Error > { let mut buffer = 0u128 . to_ne_bytes () ; let length = buffer . len () ; let bytes = match value { Value :: Bytes (bytes) => { let mut bytes : & [u8] = bytes . as_ref () ; while bytes . len () > buffer . len () && bytes [0] == 0 { bytes = & bytes [1 ..] ; } if bytes . len () > buffer . len () { return Err (de :: Error :: custom ("bigint too large")) ; } bytes } _ => return Err (de :: Error :: invalid_type (value . into () , & "bytes")) , } ; buffer [length - bytes . len () ..] . copy_from_slice (bytes) ; Ok (u128 :: from_be_bytes (buffer)) } let err = | | de :: Error :: invalid_type (self . 0 . into () , & kind) ; Ok (match self . 0 { Value :: Integer (x) => i128 :: from (* x) . try_into () . map_err (| _ | err ()) ? , Value :: Tag (t , v) if * t == tag :: BIGPOS => raw (v) ? . try_into () . map_err (| _ | err ()) ? , Value :: Tag (t , v) if * t == tag :: BIGNEG => i128 :: try_from (raw (v) ?) . map (| x | x ^ ! 0) . map_err (| _ | err ()) . and_then (| x | x . try_into () . map_err (| _ | err ())) ? , _ => return Err (de :: Error :: invalid_type (self . 0 . into () , & "(big)int")) , }) } }
};
}
