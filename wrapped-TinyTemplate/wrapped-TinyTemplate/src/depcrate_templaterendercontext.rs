// Generated macro for RenderContext (struct)
macro_rules! Depcrate_templateRenderContext {
() => {
// Module: crate::template
// Provides: {"RenderContext"}
// Dependencies: {}
# [doc = " Helper struct which mostly exists so that I have somewhere to put functions that access the"] # [doc = " rendering context stack."] struct RenderContext < 'render , 'template > { original_text : & 'template str , context_stack : Vec < ContextElement < 'render , 'template > > , }
};
}
