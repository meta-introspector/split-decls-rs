// Generated macro for impl_72 (impl)
macro_rules! Depcrate_decorators_inlineimpl_72 {
() => {
// Module: crate::decorators::inline
// Provides: {"impl_72"}
// Dependencies: {}
impl DecoratorDef for InlineDecorator { fn call < 'reg : 'rc , 'rc > (& self , d : & Decorator < 'rc > , _ : & 'reg Registry < 'reg > , _ : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> DecoratorResult { let name = get_name (d) ? ; let template = d . template () . ok_or (RenderErrorReason :: BlockContentRequired) ? ; rc . set_partial (name , template) ; Ok (()) } }
};
}
