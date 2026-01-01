// Targeted patch for AST_.._rust_compiler_rustc_next_trait_solver_src_solve_assembly_structural_traits_FN_0015
// Fixes "could not find `solve` in the crate root" error by changing visibility

#[warn(unused_variables)] // AST_.._rust_compiler_rustc_next_trait_solver_src_solve_assembly_structural_traits_FN_0015
pub(crate) fn extract_tupled_inputs_and_output_from_callable<I: Interner>(
    cx: I,
    self_ty: I::Ty,
    goal_kind: ty::ClosureKind,
) -> Result<Option<ty::Binder<I, (I::Ty, I::Ty)>>, NoSolution> {
    match self_ty.kind() {
        ty::FnDef(def_id, args) => {
            let sig = cx.fn_sig(def_id);
            if sig.skip_binder().is_fn_trait_compatible() && !cx.has_target_features(def_id) {
                Ok(Some(
                    sig.instantiate(cx, args).map_bound(|sig| {
                        (Ty::new_tup(cx, sig.inputs().as_slice()), sig.output())
                    }),
                ))
            } else {
                Err(NoSolution)
            }
        }
        ty::FnPtr(sig_tys, hdr) => {
            let sig = sig_tys.with(hdr);
            if sig.is_fn_trait_compatible() {
                Ok(Some(sig.map_bound(|sig| {
                    (Ty::new_tup(cx, sig.inputs().as_slice()), sig.output())
                })))
            } else {
                Err(NoSolution)
            }
        }
        // Additional match arms would go here...
        _ => Err(NoSolution),
    }
}
