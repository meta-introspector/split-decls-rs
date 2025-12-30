// Generated macro for impl_240 (impl)
macro_rules! Depcrate_astimpl_240 {
() => {
// Module: crate::ast
// Provides: {"impl_240"}
// Dependencies: {}
impl TemplateParam { fn resolve < 'subs , 'prev > (& 'subs self , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> :: core :: result :: Result < & 'subs TemplateArg , fmt :: Error > { scope . get_template_arg (self . 0) . map_err (| e | { log ! ("Error obtaining template argument: {}" , e) ; fmt :: Error }) . map (| v | v . 0) } }
};
}
