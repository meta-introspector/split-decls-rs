macro_rules! deps {
    () => {
        DebugLineOffset!();
        Reader!();
    };
}

macro_rules! MacroUnitHeader {
    () => {
        deps!();
        # [derive (Debug , Clone)] struct MacroUnitHeader < R : Reader > { # [doc = " The version of the macro unit header. At the moment only version 5 is defined."] _version : u16 , flags : u8 , _debug_line_offset : DebugLineOffset < R :: Offset > , }
    };
}

MacroUnitHeader!();