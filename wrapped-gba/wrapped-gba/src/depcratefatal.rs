// Generated macro for fatal (macro)
macro_rules! Depcratefatal {
() => {
// Module: crate
// Provides: {"fatal"}
// Dependencies: {}
macro_rules ! fatal { ($ ($ tt : tt) *) => { if let Ok (mut logger) = MgbaBufferedLogger :: try_new (MgbaMessageLevel :: Fatal) { use core :: fmt :: Write as _ ; let _ = writeln ! (logger , $ ($ tt) *) ; } } ; }
};
}
