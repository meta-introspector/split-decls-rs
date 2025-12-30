// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl FixedOutputCore for AsconCore { fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { debug_assert ! (buffer . get_pos () < 8) ; self . state . absorb_last_block (& buffer . get_data () [.. buffer . get_pos ()]) ; self . state . squeeze (out) ; } }
};
}
