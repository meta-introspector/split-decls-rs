macro_rules! nonnull_optimization_guaranteed {
    () => {
        pub (crate) fn nonnull_optimization_guaranteed < 'tcx > (tcx : TyCtxt < 'tcx > , def : ty :: AdtDef < 'tcx > ,) -> bool { tcx . has_attr (def . did () , sym :: rustc_nonnull_optimization_guaranteed) }
    };
}

nonnull_optimization_guaranteed!()