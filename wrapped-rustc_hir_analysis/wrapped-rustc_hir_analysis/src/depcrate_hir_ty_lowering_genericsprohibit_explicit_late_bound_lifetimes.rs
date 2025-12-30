// Generated macro for prohibit_explicit_late_bound_lifetimes (function)
macro_rules! Depcrate_hir_ty_lowering_genericsprohibit_explicit_late_bound_lifetimes {
() => {
// Module: crate::hir_ty_lowering::generics
// Provides: {"prohibit_explicit_late_bound_lifetimes"}
// Dependencies: {}
# [doc = " Prohibits explicit lifetime arguments if late-bound lifetime parameters"] # [doc = " are present. This is used both for datatypes and function calls."] pub (crate) fn prohibit_explicit_late_bound_lifetimes (cx : & dyn HirTyLowerer < '_ > , def : & ty :: Generics , args : & hir :: GenericArgs < '_ > , position : GenericArgPosition ,) -> ExplicitLateBound { let param_counts = def . own_counts () ; let infer_lifetimes = position != GenericArgPosition :: Type && ! args . has_lifetime_params () ; if infer_lifetimes { return ExplicitLateBound :: No ; } if let Some (span_late) = def . has_late_bound_regions { let msg = "cannot specify lifetime arguments explicitly \
                       if late bound lifetime parameters are present" ; let note = "the late bound lifetime parameter is introduced here" ; let span = args . args [0] . span () ; if position == GenericArgPosition :: Value && args . num_lifetime_params () != param_counts . lifetimes { struct_span_code_err ! (cx . dcx () , span , E0794 , "{}" , msg) . with_span_note (span_late , note) . emit () ; } else { let mut multispan = MultiSpan :: from_span (span) ; multispan . push_span_label (span_late , note) ; cx . tcx () . node_span_lint (LATE_BOUND_LIFETIME_ARGUMENTS , args . args [0] . hir_id () , multispan , | lint | { lint . primary_message (msg) ; } ,) ; } ExplicitLateBound :: Yes } else { ExplicitLateBound :: No } }
};
}
