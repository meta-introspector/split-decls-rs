// Generated macro for print (macro)
macro_rules! Depcrateprint {
() => {
// Module: crate
// Provides: {"print"}
// Dependencies: {}
macro_rules ! print { ($ ($ tt : tt) *) => { if let Ok (mut logger) = MgbaBufferedLogger :: try_new (MgbaMessageLevel :: Warning) { use core :: fmt :: Write as _ ; let _ = write ! (logger , $ ($ tt) *) ; } } ; }
};
}
