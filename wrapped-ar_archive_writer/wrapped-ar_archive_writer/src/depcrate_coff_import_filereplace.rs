// Generated macro for replace (function)
macro_rules! Depcrate_coff_import_filereplace {
() => {
// Module: crate::coff_import_file
// Provides: {"replace"}
// Dependencies: {}
fn replace (s : & str , mut from : & str , mut to : & str) -> Result < String > { if let Some ((before , after)) = s . split_once (from) { return Ok (format ! ("{before}{to}{after}")) ; } if from . starts_with ('_') && to . starts_with ('_') { from = & from [1 ..] ; to = & to [1 ..] ; if let Some ((before , after)) = s . split_once (from) { return Ok (format ! ("{before}{to}{after}")) ; } } Err (Error :: other (format ! ("{s}: replacing '{from}' with '{to}' failed"))) }
};
}
