// Generated macro for write_value_missing (function)
macro_rules! Depcrate_format_datetimewrite_value_missing {
() => {
// Module: crate::format::datetime
// Provides: {"write_value_missing"}
// Dependencies: {}
fn write_value_missing (w : & mut (impl writeable :: PartsWrite + ? Sized) , field : fields :: Field ,) -> Result < () , fmt :: Error > { w . with_part (Part :: ERROR , | w | { "{" . write_to (w) ? ; char :: from (field . symbol) . write_to (w) ? ; "}" . write_to (w) }) }
};
}
