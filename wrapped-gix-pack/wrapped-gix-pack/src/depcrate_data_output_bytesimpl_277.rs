// Generated macro for impl_277 (impl)
macro_rules! Depcrate_data_output_bytesimpl_277 {
() => {
// Module: crate::data::output::bytes
// Provides: {"impl_277"}
// Dependencies: {}
impl < I , W , E > Iterator for FromEntriesIter < I , W > where I : Iterator < Item = Result < Vec < output :: Entry > , E > > , W : std :: io :: Write , E : std :: error :: Error + 'static , { # [doc = " The amount of bytes written to `out` if `Ok` or the error `E` received from the input."] type Item = Result < u64 , Error < E > > ; fn next (& mut self) -> Option < Self :: Item > { if self . is_done { return None ; } Some (match self . next_inner () { Err (err) => { self . is_done = true ; Err (err) } Ok (written) => Ok (written) , }) } }
};
}
