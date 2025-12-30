// Generated macro for write_expression (function)
macro_rules! Depcrate_write_locwrite_expression {
() => {
// Module: crate::write::loc
// Provides: {"write_expression"}
// Dependencies: {}
fn write_expression < W : Writer > (w : & mut W , refs : & mut Vec < DebugInfoFixup > , encoding : Encoding , unit_offsets : Option < & UnitOffsets > , val : & Expression ,) -> Result < () > { let size = val . size (encoding , unit_offsets) ? as u64 ; if encoding . version <= 4 { w . write_udata (size , 2) ? ; } else { w . write_uleb128 (size) ? ; } val . write (w , Some (refs) , encoding , unit_offsets) ? ; Ok (()) }
};
}
