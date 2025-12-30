// Generated macro for impl_24 (impl)
macro_rules! Depcrate_control_codeimpl_24 {
() => {
// Module: crate::control_code
// Provides: {"impl_24"}
// Dependencies: {}
impl AsRef < str > for ControlCode { fn as_ref (& self) -> & str { use ControlCode :: * ; match self { Null => "^@" , StartOfHeading => "^A" , StartOfText => "^B" , EndOfText => "^C" , EndOfTransmission => "^D" , Enquiry => "^E" , Acknowledge => "^F" , Bell => "^G" , Backspace => "^H" , HorizontalTabulation => "^I" , LineFeed => "^J" , VerticalTabulation => "^K" , FormFeed => "^L" , CarriageReturn => "^M" , ShiftOut => "^N" , ShiftIn => "^O" , DataLinkEscape => "^P" , DeviceControl1 => "^Q" , DeviceControl2 => "^R" , DeviceControl3 => "^S" , DeviceControl4 => "^T" , NegativeAcknowledge => "^U" , SynchronousIdle => "^V" , EndOfTransmissionBlock => "^W" , Cancel => "^X" , EndOfMedium => "^Y" , Substitute => "^Z" , Escape => "^[" , FileSeparator => "^\\" , GroupSeparator => "^]" , RecordSeparator => "^^" , UnitSeparator => "^_" , Space => " " , Delete => "^?" , } } }
};
}
