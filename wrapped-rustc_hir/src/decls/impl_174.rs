macro_rules! deps {
    () => {
        MaybeOwner!();
        OwnerInfo!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < 'tcx > MaybeOwner < 'tcx > { pub fn as_owner (self) -> Option < & 'tcx OwnerInfo < 'tcx > > { match self { MaybeOwner :: Owner (i) => Some (i) , MaybeOwner :: NonOwner (_) | MaybeOwner :: Phantom => None , } } pub fn unwrap (self) -> & 'tcx OwnerInfo < 'tcx > { self . as_owner () . unwrap_or_else (| | panic ! ("Not a HIR owner")) } }
    };
}

impl_174!();