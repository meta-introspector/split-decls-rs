macro_rules! deps {
    () => {
        DebugCuIndex!();
        DebugTuIndex!();
        IndexSectionId!();
        Reader!();
    };
}

macro_rules! UnitIndex {
    () => {
        deps!();
        # [doc = " The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`."] # [derive (Debug , Clone)] pub struct UnitIndex < R : Reader > { version : u16 , section_count : u32 , unit_count : u32 , slot_count : u32 , hash_ids : R , hash_rows : R , sections : [IndexSectionId ; SECTION_COUNT_MAX as usize] , offsets : R , sizes : R , }
    };
}

UnitIndex!();