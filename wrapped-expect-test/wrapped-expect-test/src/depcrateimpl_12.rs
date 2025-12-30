// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl StrLitKind { fn write_start (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "r") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } write ! (w , "\"") } } } fn write_end (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "\"") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } Ok (()) } } } }
};
}
