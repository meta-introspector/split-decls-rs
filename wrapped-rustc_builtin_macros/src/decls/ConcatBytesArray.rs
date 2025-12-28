macro_rules! ConcatBytesArray {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_array)] pub (crate) struct ConcatBytesArray { # [primary_span] pub (crate) span : Span , # [note] # [help] pub (crate) bytestr : bool , }
    };
}

ConcatBytesArray!()