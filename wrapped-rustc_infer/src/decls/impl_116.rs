macro_rules! deps {
    () => {
        SccUniverse!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'tcx > SccUniverse < 'tcx > { # [doc = " If `universe` is less than our current universe, then update"] # [doc = " `self.universe` and `self.region`."] fn take_min (& mut self , universe : ty :: UniverseIndex , region : ty :: Region < 'tcx >) { if universe < self . universe || self . region . is_none () { self . universe = universe ; self . region = Some (region) ; } } }
    };
}

impl_116!()