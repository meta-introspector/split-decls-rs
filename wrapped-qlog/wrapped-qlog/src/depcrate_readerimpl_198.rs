// Generated macro for impl_198 (impl)
macro_rules! Depcrate_readerimpl_198 {
() => {
// Module: crate::reader
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a > QlogSeqReader < 'a > { pub fn new (mut reader : Box < dyn std :: io :: BufRead + Send + Sync + 'a > ,) -> Result < Self , Box < dyn std :: error :: Error > > { Self :: read_record (reader . as_mut ()) ; let header = Self :: read_record (reader . as_mut ()) . ok_or_else (| | { std :: io :: Error :: other ("error reading file header bytes") }) ? ; let res : Result < QlogSeq , serde_json :: Error > = serde_json :: from_slice (& header) ; match res { Ok (qlog) => Ok (Self { qlog , reader }) , Err (e) => Err (e . into ()) , } } fn read_record (reader : & mut (dyn std :: io :: BufRead + Send + Sync) ,) -> Option < Vec < u8 > > { let mut buf = Vec :: < u8 > :: new () ; let size = reader . read_until (b'' , & mut buf) . unwrap () ; if size <= 1 { return None ; } buf . truncate (buf . len () - 1) ; Some (buf) } }
};
}
