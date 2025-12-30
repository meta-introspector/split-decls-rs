// Generated macro for impl_221 (impl)
macro_rules! Depcrate_helpers_helper_rawimpl_221 {
() => {
// Module: crate::helpers::helper_raw
// Provides: {"impl_221"}
// Dependencies: {}
impl HelperDef for RawHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { let tpl = h . template () ; if let Some (t) = tpl { t . render (r , ctx , rc , out) } else { Ok (()) } } }
};
}
