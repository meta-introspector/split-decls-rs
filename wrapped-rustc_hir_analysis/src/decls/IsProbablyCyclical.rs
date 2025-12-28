macro_rules! IsProbablyCyclical {
    () => {
        # [doc = " Detects cases where an ADT/LTA is trivially cyclical -- we want to detect this so"] # [doc = " we only mention that its parameters are used cyclically if the ADT/LTA is truly"] # [doc = " cyclical."] # [doc = ""] # [doc = " Notably, we don't consider substitutions here, so this may have false positives."] struct IsProbablyCyclical < 'tcx > { tcx : TyCtxt < 'tcx > , item_def_id : DefId , seen : FxHashSet < DefId > , }
    };
}

IsProbablyCyclical!()