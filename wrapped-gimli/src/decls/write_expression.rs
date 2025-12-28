macro_rules! deps {
    () => {
        Encoding!();
        UnitOffsets!();
        Expression!();
        DebugInfoFixup!();
        Result!();
        Writer!();
    };
}

macro_rules! write_expression {
    () => {
        deps!();
        fn write_expression < W : Writer > (w : & mut W , refs : & mut Vec < DebugInfoFixup > , encoding : Encoding , unit_offsets : Option < & UnitOffsets > , val : & Expression ,) -> Result < () > { let size = val . size (encoding , unit_offsets) ? as u64 ; if encoding . version <= 4 { w . write_udata (size , 2) ? ; } else { w . write_uleb128 (size) ? ; } val . write (w , Some (refs) , encoding , unit_offsets) ? ; Ok (()) }
    };
}

write_expression!()