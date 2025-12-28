macro_rules! deps {
    () => {
        TyCtxtConsts!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < 'tcx > TyCtxtConsts < 'tcx > { const DEREF_PROJECTION : & 'tcx [PlaceElem < 'tcx > ; 1] = & [ProjectionElem :: Deref] ; }
    };
}

impl_517!()