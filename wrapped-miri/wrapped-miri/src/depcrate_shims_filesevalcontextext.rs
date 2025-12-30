// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_shims_filesEvalContextExt {
() => {
// Module: crate::shims::files
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { # [doc = " Read data from a host `Read` type, store the result into machine memory,"] # [doc = " and return whether that worked."] fn read_from_host (& mut self , mut file : impl io :: Read , len : usize , ptr : Pointer ,) -> InterpResult < 'tcx , Result < usize , IoError > > { let this = self . eval_context_mut () ; let mut bytes = vec ! [0 ; len] ; let result = file . read (& mut bytes) ; match result { Ok (read_size) => { this . write_bytes_ptr (ptr , bytes [.. read_size] . iter () . copied ()) ? ; interp_ok (Ok (read_size)) } Err (e) => interp_ok (Err (IoError :: HostError (e))) , } } # [doc = " Write data to a host `Write` type, withthe bytes taken from machine memory."] fn write_to_host (& mut self , mut file : impl io :: Write , len : usize , ptr : Pointer ,) -> InterpResult < 'tcx , Result < usize , IoError > > { let this = self . eval_context_mut () ; let bytes = this . read_bytes_ptr_strip_provenance (ptr , Size :: from_bytes (len)) ? ; let result = file . write (bytes) ; interp_ok (result . map_err (IoError :: HostError)) } }
};
}
