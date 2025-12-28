macro_rules! try_fn_sig {
    () => {
        # [doc = " In some cases, attributes are only valid on functions, but it's the `check_attr`"] # [doc = " pass that checks that they aren't used anywhere else, rather than this module."] # [doc = " In these cases, we bail from performing further checks that are only meaningful for"] # [doc = " functions (such as calling `fn_sig`, which ICEs if given a non-function). We also"] # [doc = " report a delayed bug, just in case `check_attr` isn't doing its job."] fn try_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , did : LocalDefId , attr_span : Span ,) -> Option < ty :: EarlyBinder < 'tcx , ty :: PolyFnSig < 'tcx > > > { use DefKind :: * ; let def_kind = tcx . def_kind (did) ; if let Fn | AssocFn | Variant | Ctor (..) = def_kind { Some (tcx . fn_sig (did)) } else { tcx . dcx () . span_delayed_bug (attr_span , "this attribute can only be applied to functions") ; None } }
    };
}

try_fn_sig!();