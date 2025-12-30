// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_evcxrimpl_1316 {
() => {
// Module: crate::evcxr
// Provides: {"impl_1316"}
// Dependencies: {}
impl std :: fmt :: Debug for SVGWrapper { fn fmt (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let svg = self . 0 . as_str () ; write ! (formatter , "EVCXR_BEGIN_CONTENT text/html\n<div style=\"{}\">{}</div>\nEVCXR_END_CONTENT" , self . 1 , svg) } }
};
}
