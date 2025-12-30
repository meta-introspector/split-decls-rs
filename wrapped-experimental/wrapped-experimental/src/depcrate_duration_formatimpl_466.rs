// Generated macro for impl_466 (impl)
macro_rules! Depcrate_duration_formatimpl_466 {
() => {
// Module: crate::duration::format
// Provides: {"impl_466"}
// Dependencies: {}
impl Writeable for FormattedDigitalDuration < '_ > { fn write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S) -> fmt :: Result { if let Some (hours) = & self . hours { sink . with_part (parts :: HOUR , | w | hours . write_to_parts (w)) ? ; } if self . add_hour_minute_separator { sink . with_part (parts :: LITERAL , | w | { w . write_str (& self . fmt . digital . get () . separator) }) ? ; } if let Some (minutes) = & self . minutes { sink . with_part (parts :: MINUTE , | w | minutes . write_to_parts (w)) ? ; } if self . add_minute_second_separator { sink . with_part (parts :: LITERAL , | w | { w . write_str (& self . fmt . digital . get () . separator) }) ? ; } if let Some (seconds) = & self . seconds { sink . with_part (parts :: SECOND , | w | seconds . write_to_parts (w)) ? ; } Ok (()) } }
};
}
