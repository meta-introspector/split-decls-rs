// Generated macro for impl_22 (impl)
macro_rules! Depcrate_control_codeimpl_22 {
() => {
// Module: crate::control_code
// Provides: {"impl_22"}
// Dependencies: {}
impl TryFrom < char > for ControlCode { type Error = () ; fn try_from (c : char) -> Result < ControlCode , () > { use ControlCode :: * ; match c { '@' => Ok (Null) , 'A' | 'a' => Ok (StartOfHeading) , 'B' | 'b' => Ok (StartOfText) , 'C' | 'c' => Ok (EndOfText) , 'D' | 'd' => Ok (EndOfTransmission) , 'E' | 'e' => Ok (Enquiry) , 'F' | 'f' => Ok (Acknowledge) , 'G' | 'g' => Ok (Bell) , 'H' | 'h' => Ok (Backspace) , 'I' | 'i' => Ok (HorizontalTabulation) , 'J' | 'j' => Ok (LineFeed) , 'K' | 'k' => Ok (VerticalTabulation) , 'L' | 'l' => Ok (FormFeed) , 'M' | 'm' => Ok (CarriageReturn) , 'N' | 'n' => Ok (ShiftOut) , 'O' | 'o' => Ok (ShiftIn) , 'P' | 'p' => Ok (DataLinkEscape) , 'Q' | 'q' => Ok (DeviceControl1) , 'R' | 'r' => Ok (DeviceControl2) , 'S' | 's' => Ok (DeviceControl3) , 'T' | 't' => Ok (DeviceControl4) , 'U' | 'u' => Ok (NegativeAcknowledge) , 'V' | 'v' => Ok (SynchronousIdle) , 'W' | 'w' => Ok (EndOfTransmissionBlock) , 'X' | 'x' => Ok (Cancel) , 'Y' | 'y' => Ok (EndOfMedium) , 'Z' | 'z' => Ok (Substitute) , '[' => Ok (Escape) , '\\' => Ok (FileSeparator) , ']' => Ok (GroupSeparator) , '^' => Ok (RecordSeparator) , '_' => Ok (UnitSeparator) , ' ' => Ok (Space) , '?' => Ok (Delete) , _ => Err (()) , } } }
};
}
