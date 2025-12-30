// Generated macro for impl_325 (impl)
macro_rules! Depcrateimpl_325 {
() => {
// Module: crate
// Provides: {"impl_325"}
// Dependencies: {}
impl ConcatStreamsHelper { fn new (capacity : usize) -> Self { ConcatStreamsHelper { streams : Vec :: with_capacity (capacity) } } fn push (& mut self , stream : TokenStream) { if let Some (stream) = stream . 0 { self . streams . push (stream) ; } } fn build (mut self) -> TokenStream { if self . streams . len () <= 1 { TokenStream (self . streams . pop ()) } else { TokenStream (Some (bridge :: client :: TokenStream :: concat_streams (None , self . streams))) } } fn append_to (mut self , stream : & mut TokenStream) { if self . streams . is_empty () { return ; } let base = stream . 0 . take () ; if base . is_none () && self . streams . len () == 1 { stream . 0 = self . streams . pop () ; } else { stream . 0 = Some (bridge :: client :: TokenStream :: concat_streams (base , self . streams)) ; } } }
};
}
