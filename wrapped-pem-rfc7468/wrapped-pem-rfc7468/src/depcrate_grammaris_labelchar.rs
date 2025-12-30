// Generated macro for is_labelchar (function)
macro_rules! Depcrate_grammaris_labelchar {
() => {
// Module: crate::grammar
// Provides: {"is_labelchar"}
// Dependencies: {}
# [doc = " Any printable character except hyphen-minus, as defined in the"] # [doc = " 'labelchar' production in the RFC 7468 ABNF grammar"] pub (crate) fn is_labelchar (char : u8) -> bool { matches ! (char , 0x21 ..= 0x2C | 0x2E ..= 0x7E) }
};
}
