// Generated macro for try_append_to_string (function)
macro_rules! Depcrate_formattry_append_to_string {
() => {
// Module: crate::format
// Provides: {"try_append_to_string"}
// Dependencies: {}
# [doc = " Appends the data to the string."] # [doc = ""] # [doc = " When allocation failure happens, incompletely appended strings won't be"] # [doc = " stripped. Callers are responsible to clean up the destination if necessary."] # [cfg (feature = "alloc")] pub fn try_append_to_string < T : fmt :: Display > (dest : & mut String , value : & T ,) -> Result < () , TryReserveError > { let mut writer = StringWriter { buffer : dest , error : None , } ; if write ! (writer , "{}" , value) . is_err () { let e = writer . error . expect ("[consistency] allocation error should be set on formatting failure") ; return Err (e) ; } Ok (()) }
};
}
