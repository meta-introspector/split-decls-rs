// Generated macro for try_read_vlq (macro)
macro_rules! Depcrate_decodetry_read_vlq {
() => {
// Module: crate::decode
// Provides: {"try_read_vlq"}
// Dependencies: {}
macro_rules ! try_read_vlq { ($ data : expr) => { match read_vlq (& mut $ data) { Ok (v) => v , Err (err) => return Err (err) , } } ; }
};
}
