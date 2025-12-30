// Generated macro for impl_793 (impl)
macro_rules! Depcrate_relativetime_formatimpl_793 {
() => {
// Module: crate::relativetime::format
// Provides: {"impl_793"}
// Dependencies: {}
impl Writeable for FormattedRelativeTime < '_ > { fn write_to_parts < S : writeable :: PartsWrite + ? Sized > (& self , sink : & mut S) -> core :: fmt :: Result { if self . options . numeric == Numeric :: Auto { let relatives = & self . formatter . rt . get () . relatives ; if self . value . absolute . magnitude_range () == (0 ..= 0) { let i8_value = if self . is_negative { - (self . value . absolute . digit_at (0) as i8) } else { self . value . absolute . digit_at (0) as i8 } ; if let Some (v) = relatives . get (& i8_value) { sink . with_part (parts :: LITERAL , | s | s . write_str (v)) ? ; return Ok (()) ; } } } if self . is_negative { & self . formatter . rt . get () . past } else { & self . formatter . rt . get () . future } . get ((& self . value) . into () , & self . formatter . plural_rules) . interpolate ((self . formatter . decimal_formatter . format (& self . value) ,)) . write_to (sink) } }
};
}
