// Generated macro for WriteValue (trait)
macro_rules! Depcrate_resolverWriteValue {
() => {
// Module: crate::resolver
// Provides: {"WriteValue"}
// Dependencies: {}
# [doc = " Resolves an AST node to a string that is written to source `W`."] pub (crate) trait WriteValue < 'bundle > { # [doc = " Resolves an AST node to a string that is written to source `W`."] fn write < 'ast , 'args , 'errors , W , R , M > (& 'ast self , w : & mut W , scope : & mut Scope < 'bundle , 'ast , 'args , 'errors , R , M > ,) -> fmt :: Result where W : fmt :: Write , R : Borrow < FluentResource > , M : MemoizerKind ; # [doc = " Writes error information to `W`. This can be used to add FTL errors inline"] # [doc = " to a message."] fn write_error < W > (& self , _w : & mut W) -> fmt :: Result where W : fmt :: Write ; }
};
}
