// Generated macro for visit_inline_fragment (function)
macro_rules! Depcrate_validation_visitorvisit_inline_fragment {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_inline_fragment"}
// Dependencies: {}
fn visit_inline_fragment < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , fragment : & 'a Spanning < InlineFragment < 'a , S > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { let mut visit_fn = move | ctx : & mut ValidatorContext < 'a , S > | { v . enter_inline_fragment (ctx , fragment) ; visit_directives (v , ctx , & fragment . item . directives) ; visit_selection_set (v , ctx , & fragment . item . selection_set) ; v . exit_inline_fragment (ctx , fragment) ; } ; if let Some (Spanning { item : type_name , .. }) = fragment . item . type_condition { ctx . with_pushed_type (Some (BorrowedType :: non_null (type_name)) , visit_fn) ; } else { visit_fn (ctx) ; } }
};
}
