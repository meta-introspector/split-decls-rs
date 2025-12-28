macro_rules! HasErrorDeep {
    () => {
        # [doc = " Look for `ErrorGuaranteed` deeply within structs' (unsubstituted) fields."] struct HasErrorDeep < 'tcx > { tcx : TyCtxt < 'tcx > , seen : FxHashSet < DefId > , }
    };
}

HasErrorDeep!();