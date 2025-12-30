// Generated macro for impl_252 (impl)
macro_rules! Depcrate_helpers_scriptingimpl_252 {
() => {
// Module: crate::helpers::scripting
// Provides: {"impl_252"}
// Dependencies: {}
impl HelperDef for ScriptHelper { fn call_inner < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , reg : & 'reg Registry < 'reg > , _ctx : & 'rc Context , _rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < ScopedJson < 'rc > , RenderError > { call_script_helper (h . params () , h . hash () , & reg . engine , & self . script) } }
};
}
