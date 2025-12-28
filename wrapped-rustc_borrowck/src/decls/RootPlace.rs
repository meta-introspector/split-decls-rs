macro_rules! deps {
    () => {
        LocalMutationIsAllowed!();
    };
}

macro_rules! RootPlace {
    () => {
        deps!();
        # [derive (Debug)] struct RootPlace < 'tcx > { place_local : Local , place_projection : & 'tcx [PlaceElem < 'tcx >] , is_local_mutation_allowed : LocalMutationIsAllowed , }
    };
}

RootPlace!();