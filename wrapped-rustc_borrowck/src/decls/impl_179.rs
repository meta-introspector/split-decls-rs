macro_rules! deps {
    () => {
        SccAnnotations!();
        RegionDefinition!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'd , 'tcx , A : scc :: Annotation > SccAnnotations < 'd , 'tcx , A > { pub (crate) fn init (definitions : & 'd IndexVec < RegionVid , RegionDefinition < 'tcx > >) -> Self { Self { scc_to_annotation : IndexVec :: new () , definitions } } }
    };
}

impl_179!()