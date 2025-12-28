macro_rules! TypeRelativePath {
    () => {
        # [derive (Debug , Clone , Copy)] enum TypeRelativePath < 'tcx > { AssocItem (DefId , GenericArgsRef < 'tcx >) , Variant { adt : Ty < 'tcx > , variant_did : DefId } , }
    };
}

TypeRelativePath!();