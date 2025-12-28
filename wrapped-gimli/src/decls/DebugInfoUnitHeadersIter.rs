macro_rules! deps {
    () => {
        UnitSectionOffset!();
        Reader!();
    };
}

macro_rules! DebugInfoUnitHeadersIter {
    () => {
        deps!();
        # [doc = " An iterator over the units of a .debug_info section."] # [doc = ""] # [doc = " See the [documentation on"] # [doc = " `DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail."] # [derive (Clone , Debug)] pub struct DebugInfoUnitHeadersIter < R : Reader > { input : R , offset : UnitSectionOffset < R :: Offset > , }
    };
}

DebugInfoUnitHeadersIter!()