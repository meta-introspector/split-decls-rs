macro_rules! deps {
    () => {
        TraitOrTraitImpl!();
        TildeConstReason!();
    };
}

macro_rules! AstValidator {
    () => {
        deps!();
        struct AstValidator < 'a > { sess : & 'a Session , features : & 'a Features , # [doc = " The span of the `extern` in an `extern { ... }` block, if any."] extern_mod_span : Option < Span > , outer_trait_or_trait_impl : Option < TraitOrTraitImpl > , has_proc_macro_decls : bool , # [doc = " Used to ban nested `impl Trait`, e.g., `impl Into<impl Debug>`."] # [doc = " Nested `impl Trait` _is_ allowed in associated type position,"] # [doc = " e.g., `impl Iterator<Item = impl Debug>`."] outer_impl_trait_span : Option < Span > , disallow_tilde_const : Option < TildeConstReason > , # [doc = " Used to ban explicit safety on foreign items when the extern block is not marked as unsafe."] extern_mod_safety : Option < Safety > , extern_mod_abi : Option < ExternAbi > , lint_node_id : NodeId , is_sdylib_interface : bool , lint_buffer : & 'a mut LintBuffer , }
    };
}

AstValidator!()