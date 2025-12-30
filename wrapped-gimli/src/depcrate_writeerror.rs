// Generated macro for Error (enum)
macro_rules! Depcrate_writeError {
() => {
// Module: crate::write
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurred when writing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Error { # [doc = " The given offset is out of bounds."] OffsetOutOfBounds , # [doc = " The given length is out of bounds."] LengthOutOfBounds , # [doc = " The attribute value is an invalid for writing."] InvalidAttributeValue , # [doc = " The value is too large for the encoding form."] ValueTooLarge , # [doc = " Unsupported word size."] UnsupportedWordSize (u8) , # [doc = " Unsupported DWARF version."] UnsupportedVersion (u16) , # [doc = " The unit length is too large for the requested DWARF format."] InitialLengthOverflow , # [doc = " The address is invalid."] InvalidAddress , # [doc = " The reference is invalid."] InvalidReference , # [doc = " A requested feature requires a different DWARF version."] NeedVersion (u16) , # [doc = " Strings in line number program have mismatched forms."] LineStringFormMismatch , # [doc = " The range is empty or otherwise invalid."] InvalidRange , # [doc = " The line number program encoding is incompatible with the unit encoding."] IncompatibleLineProgramEncoding , # [doc = " Could not encode code offset for a frame instruction."] InvalidFrameCodeOffset (u32) , # [doc = " Could not encode data offset for a frame instruction."] InvalidFrameDataOffset (i32) , # [doc = " Unsupported eh_frame pointer encoding."] UnsupportedPointerEncoding (constants :: DwEhPe) , # [doc = " Unsupported reference in CFI expression."] UnsupportedCfiExpressionReference , # [doc = " Unsupported forward reference in expression."] UnsupportedExpressionForwardReference , }
};
}
