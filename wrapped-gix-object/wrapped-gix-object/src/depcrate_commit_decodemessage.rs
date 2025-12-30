// Generated macro for message (function)
macro_rules! Depcrate_commit_decodemessage {
() => {
// Module: crate::commit::decode
// Provides: {"message"}
// Dependencies: {}
pub fn message < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < & 'a BStr , E > { if i . is_empty () { let start = i . checkpoint () ; return Err (winnow :: error :: ErrMode :: from_input (i) . add_context (i , & start , StrContext :: Expected ("newline + <message>" . into ()) ,)) ; } preceded (NL , rest . map (ByteSlice :: as_bstr)) . context (StrContext :: Expected ("a newline separates headers from the message" . into () ,)) . parse_next (i) }
};
}
