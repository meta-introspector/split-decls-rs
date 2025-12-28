macro_rules! deps {
    () => {
        DebugInfoOffset!();
        UnitOffsets!();
    };
}

macro_rules! impl_812 {
    () => {
        deps!();
        impl UnitOffsets { # [doc = " Get the `.debug_info` offset for the given entry."] # [doc = ""] # [doc = " Returns `None` if the offset has not been calculated yet."] # [inline] fn debug_info_offset (& self , entry : UnitEntryId) -> Option < DebugInfoOffset > { debug_assert_eq ! (self . base_id , entry . base_id) ; let offset = self . entries [entry . index] ; if offset . 0 == 0 { None } else { Some (offset) } } # [doc = " Get the unit offset for the given entry."] # [doc = ""] # [doc = " Returns `None` if the offset has not been calculated yet."] # [doc = " This may occur if the entry is orphaned or if a reference"] # [doc = " to the entry occurs before the entry itself is written."] # [inline] pub (crate) fn unit_offset (& self , entry : UnitEntryId) -> Option < u64 > { self . debug_info_offset (entry) . map (| offset | (offset . 0 - self . unit . 0) as u64) } }
    };
}

impl_812!()