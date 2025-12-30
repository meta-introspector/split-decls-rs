// Generated macro for utils (module)
macro_rules! Depcrateutils {
() => {
// Module: crate
// Provides: {"utils"}
// Dependencies: {}
mod utils { use std :: fmt ; pub (crate) struct Dbg < T > (pub T) ; impl < T > fmt :: Debug for Dbg < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "...") } } }
};
}
