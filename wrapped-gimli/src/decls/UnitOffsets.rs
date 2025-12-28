macro_rules! deps {
    () => {
        BaseId!();
        DebugInfoOffset!();
    };
}

macro_rules! UnitOffsets {
    () => {
        deps!();
        # [doc = " The section offsets of all elements of a unit within a `.debug_info` section."] # [derive (Debug)] pub (crate) struct UnitOffsets { base_id : BaseId , unit : DebugInfoOffset , entries : Vec < DebugInfoOffset > , }
    };
}

UnitOffsets!();