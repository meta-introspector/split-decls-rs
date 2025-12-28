macro_rules! deps {
    () => {
        RegionRelations!();
        FreeRegionMap!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a , 'tcx > RegionRelations < 'a , 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , free_regions : & 'a FreeRegionMap < 'tcx >) -> Self { Self { tcx , free_regions } } pub (crate) fn lub_param_regions (& self , r_a : Region < 'tcx > , r_b : Region < 'tcx >) -> Region < 'tcx > { self . free_regions . lub_param_regions (self . tcx , r_a , r_b) } }
    };
}

impl_51!();