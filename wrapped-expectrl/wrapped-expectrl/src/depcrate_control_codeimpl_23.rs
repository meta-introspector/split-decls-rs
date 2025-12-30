// Generated macro for impl_23 (impl)
macro_rules! Depcrate_control_codeimpl_23 {
() => {
// Module: crate::control_code
// Provides: {"impl_23"}
// Dependencies: {}
impl TryFrom < & str > for ControlCode { type Error = () ; fn try_from (c : & str) -> Result < ControlCode , () > { use ControlCode :: * ; match c { "^@" => Ok (Null) , "^A" => Ok (StartOfHeading) , "^B" => Ok (StartOfText) , "^C" => Ok (EndOfText) , "^D" => Ok (EndOfTransmission) , "^E" => Ok (Enquiry) , "^F" => Ok (Acknowledge) , "^G" => Ok (Bell) , "^H" => Ok (Backspace) , "^I" => Ok (HorizontalTabulation) , "^J" => Ok (LineFeed) , "^K" => Ok (VerticalTabulation) , "^L" => Ok (FormFeed) , "^M" => Ok (CarriageReturn) , "^N" => Ok (ShiftOut) , "^O" => Ok (ShiftIn) , "^P" => Ok (DataLinkEscape) , "^Q" => Ok (DeviceControl1) , "^R" => Ok (DeviceControl2) , "^S" => Ok (DeviceControl3) , "^T" => Ok (DeviceControl4) , "^U" => Ok (NegativeAcknowledge) , "^V" => Ok (SynchronousIdle) , "^W" => Ok (EndOfTransmissionBlock) , "^X" => Ok (Cancel) , "^Y" => Ok (EndOfMedium) , "^Z" => Ok (Substitute) , "^[" => Ok (Escape) , "^\\" => Ok (FileSeparator) , "^]" => Ok (GroupSeparator) , "^^" => Ok (RecordSeparator) , "^_" => Ok (UnitSeparator) , "^ " => Ok (Space) , "^?" => Ok (Delete) , _ => Err (()) , } } }
};
}
