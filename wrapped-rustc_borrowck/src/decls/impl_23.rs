macro_rules! deps {
    () => {
        TyCtxtConsts!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'tcx > TyCtxtConsts < 'tcx > { const DEREF_PROJECTION : & 'tcx [PlaceElem < 'tcx > ; 1] = & [ProjectionElem :: Deref] ; }
    };
}

impl_23!()