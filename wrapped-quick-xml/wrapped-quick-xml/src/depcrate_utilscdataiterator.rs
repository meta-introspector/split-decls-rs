// Generated macro for CDataIterator (struct)
macro_rules! Depcrate_utilsCDataIterator {
() => {
// Module: crate::utils
// Provides: {"CDataIterator"}
// Dependencies: {}
# [doc = " Splits string into pieces which can be part of a single `CDATA` section."] # [doc = ""] # [doc = " Because CDATA cannot contain the `]]>` sequence, split the string between"] # [doc = " `]]` and `>`."] # [derive (Debug , Clone)] pub (crate) struct CDataIterator < 'a > { # [doc = " The unprocessed data which should be emitted as `BytesCData` events."] # [doc = " At each iteration, the processed data is cut from this slice."] unprocessed : & 'a str , finished : bool , }
};
}
