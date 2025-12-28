macro_rules! deps {
    () => {
        PlaceElem!();
        ProjectionId!();
    };
}

macro_rules! ProjectionStore {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct ProjectionStore < 'db > { id_to_proj : FxHashMap < ProjectionId , Box < [PlaceElem < 'db >] > > , proj_to_id : FxHashMap < Box < [PlaceElem < 'db >] > , ProjectionId > , }
    };
}

ProjectionStore!()