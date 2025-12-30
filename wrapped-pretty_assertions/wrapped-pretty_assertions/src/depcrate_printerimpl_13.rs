// Generated macro for impl_13 (impl)
macro_rules! Depcrate_printerimpl_13 {
() => {
// Module: crate::printer
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > LatentDeletion < 'a > { # [doc = " Set the chunk value."] fn set (& mut self , value : & 'a str) { self . value = Some (value) ; self . count += 1 ; } # [doc = " Take the underlying chunk value, if it's suitable for inline diffing."] # [doc = ""] # [doc = " If there is no value or we've seen more than one line, return `None`."] fn take (& mut self) -> Option < & 'a str > { if self . count == 1 { self . value . take () } else { None } } # [doc = " If a value is set, print it as a whole chunk, using the given formatter."] # [doc = ""] # [doc = " If a value is not set, reset the count to zero (as we've called `flush` twice,"] # [doc = " without seeing another deletion. Therefore the line in the middle was something else)."] fn flush < TWrite : fmt :: Write > (& mut self , f : & mut TWrite) -> fmt :: Result { if let Some (value) = self . value { paint ! (f , Red , "{}{}" , SIGN_LEFT , value) ? ; writeln ! (f) ? ; self . value = None ; } else { self . count = 0 ; } Ok (()) } }
};
}
