macro_rules! ComplexMemoryMap {
    () => {
        # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct ComplexMemoryMap < 'db > { memory : IndexMap < usize , Box < [u8] > , FxBuildHasher > , vtable : VTableMap < 'db > , }
    };
}

ComplexMemoryMap!()