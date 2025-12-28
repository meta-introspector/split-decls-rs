macro_rules! deps {
    () => {
        Reader!();
        UnitSectionOffset!();
    };
}

macro_rules! DebugTypesUnitHeadersIter {
    () => {
        deps!();
        # [doc = " An iterator over the type-units of this `.debug_types` section."] # [doc = ""] # [doc = " See the [documentation on"] # [doc = " `DebugTypes::units`](./struct.DebugTypes.html#method.units) for"] # [doc = " more detail."] # [derive (Clone , Debug)] pub struct DebugTypesUnitHeadersIter < R : Reader > { input : R , offset : UnitSectionOffset < R :: Offset > , }
    };
}

DebugTypesUnitHeadersIter!();