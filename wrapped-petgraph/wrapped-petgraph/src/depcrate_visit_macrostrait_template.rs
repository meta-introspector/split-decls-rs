// Generated macro for trait_template (macro)
macro_rules! Depcrate_visit_macrostrait_template {
() => {
// Module: crate::visit::macros
// Provides: {"trait_template"}
// Dependencies: {}
# [doc = " Define a trait as usual, and a macro that can be used to instantiate"] # [doc = " implementations of it."] # [doc = ""] # [doc = " There *must* be section markers in the trait definition:"] # [doc = " @section type for associated types"] # [doc = " @section self for methods"] # [doc = " @section nodelegate for arbitrary tail that is not forwarded."] macro_rules ! trait_template { ($ (# [$ doc : meta]) * pub trait $ name : ident $ ($ methods : tt) *) => { macro_rules ! $ name { ($ m : ident $ extra : tt) => { $ m ! { $ extra pub trait $ name $ ($ methods) * } } } remove_sections ! { [] $ (# [$ doc]) * pub trait $ name $ ($ methods) * } } }
};
}
