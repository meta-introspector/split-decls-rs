macro_rules! DebugStrOffsets {
    () => {
        # [doc = " The raw contents of the `.debug_str_offsets` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugStrOffsets < R > { section : R , }
    };
}

DebugStrOffsets!()