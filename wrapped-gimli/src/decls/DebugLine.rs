macro_rules! DebugLine {
    () => {
        # [doc = " The `DebugLine` struct contains the source location to instruction mapping"] # [doc = " found in the `.debug_line` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugLine < R > { debug_line_section : R , }
    };
}

DebugLine!()