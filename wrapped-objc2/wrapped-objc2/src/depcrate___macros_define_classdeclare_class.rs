// Generated macro for declare_class (macro)
macro_rules! Depcrate___macros_define_classdeclare_class {
() => {
// Module: crate::__macros::define_class
// Provides: {"declare_class"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! declare_class { { $ (# [$ m : meta]) * $ v : vis struct $ name : ident ; unsafe impl ClassType for $ for_class : ty { $ (# [inherits ($ ($ inheritance_rest : ty) ,+)]) ? type Super = $ superclass : ty ; type Mutability = $ mutability : ty ; const NAME : &'static str = $ name_const : expr ; } impl DefinedClass for $ for_defined : ty { $ (type Ivars = $ ivars : ty ;) ? } $ ($ impls : tt) * } => { $ (# [$ m]) * $ v struct $ name ; $ crate :: __macros :: compile_error ! ("declare_class! has been renamed to define_class!, and the syntax has changed") } }
};
}
