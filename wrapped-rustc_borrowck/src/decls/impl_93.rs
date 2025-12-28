macro_rules! deps {
    () => {
        InstantiateOpaqueType!();
        ToUniverseInfo!();
        UniverseInfo!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'tcx > ToUniverseInfo < 'tcx > for crate :: type_check :: InstantiateOpaqueType < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (crate :: type_check :: InstantiateOpaqueType { base_universe : Some (base_universe) , .. self })) } }
    };
}

impl_93!();