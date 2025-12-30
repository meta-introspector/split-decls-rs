// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl ExtendableOutputCore for AsconXofCore { type ReaderCore = AsconXofReaderCore ; fn finalize_xof_core (& mut self , buffer : & mut Buffer < Self >) -> Self :: ReaderCore { debug_assert ! (buffer . get_pos () < 8) ; self . state . absorb_last_block (& buffer . get_data () [.. buffer . get_pos ()]) ; Self :: ReaderCore { hasher : self . state . clone () , } } }
};
}
