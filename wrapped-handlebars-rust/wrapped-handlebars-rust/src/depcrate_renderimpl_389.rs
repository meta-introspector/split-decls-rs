// Generated macro for impl_389 (impl)
macro_rules! Depcrate_renderimpl_389 {
() => {
// Module: crate::render
// Provides: {"impl_389"}
// Dependencies: {}
impl fmt :: Debug for RenderContext < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("RenderContextInner") . field ("dev_mode_templates" , & self . dev_mode_templates) . field ("blocks" , & self . blocks) . field ("modified_context" , & self . modified_context) . field ("partials" , & self . partials) . field ("partial_block_stack" , & self . partial_block_stack) . field ("partial_block_depth" , & self . partial_block_depth) . field ("root_template" , & self . root_template) . field ("current_template" , & self . current_template) . field ("disable_escape" , & self . disable_escape) . finish () } }
};
}
