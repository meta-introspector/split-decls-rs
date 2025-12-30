// Generated macro for impl_271 (impl)
macro_rules! Depcrate_back_linkerimpl_271 {
() => {
// Module: crate::back::linker
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'a > WasmLd < 'a > { fn push_linker_plugin_lto_args (& mut self) { let opt_level = match self . sess . opts . optimize { config :: OptLevel :: No => "O0" , config :: OptLevel :: Less => "O1" , config :: OptLevel :: More => "O2" , config :: OptLevel :: Aggressive => "O3" , config :: OptLevel :: Size | config :: OptLevel :: SizeMin => "O2" , } ; self . link_arg (& format ! ("--lto-{opt_level}")) ; } }
};
}
