// Generated macro for Parser (struct)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " Const-friendly OID string parser."] # [doc = ""] # [doc = " Parses an OID from the dotted string representation."] # [derive (Debug)] pub (crate) struct Parser { # [doc = " Current arc in progress"] current_arc : Option < Arc > , # [doc = " BER/DER encoder"] encoder : Encoder < { ObjectIdentifier :: MAX_SIZE } > , }
};
}
