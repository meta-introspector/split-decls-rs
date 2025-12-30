// Generated macro for macro_7407 (macro)
macro_rules! Depcrate_mismatching_type_param_ordermacro_7407 {
() => {
// Module: crate::mismatching_type_param_order
// Provides: {"macro_7407"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for type parameters which are positioned inconsistently between"] # [doc = " a type definition and impl block. Specifically, a parameter in an impl"] # [doc = " block which has the same name as a parameter in the type def, but is in"] # [doc = " a different place."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Type parameters are determined by their position rather than name."] # [doc = " Naming type parameters inconsistently may cause you to refer to the"] # [doc = " wrong type parameter."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " This lint only applies to impl blocks with simple generic params, e.g."] # [doc = " `A`. If there is anything more complicated, such as a tuple, it will be"] # [doc = " ignored."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo<A, B> {"] # [doc = "     x: A,"] # [doc = "     y: B,"] # [doc = " }"] # [doc = " // inside the impl, B refers to Foo::A"] # [doc = " impl<B, A> Foo<B, A> {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Foo<A, B> {"] # [doc = "     x: A,"] # [doc = "     y: B,"] # [doc = " }"] # [doc = " impl<A, B> Foo<A, B> {}"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub MISMATCHING_TYPE_PARAM_ORDER , pedantic , "type parameter positioned inconsistently between type def and impl block" }
};
}
