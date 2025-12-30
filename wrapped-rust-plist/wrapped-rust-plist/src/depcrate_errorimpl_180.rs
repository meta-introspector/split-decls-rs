// Generated macro for impl_180 (impl)
macro_rules! Depcrate_errorimpl_180 {
() => {
// Module: crate::error
// Provides: {"impl_180"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (position) = & self . inner . file_position { write ! (f , "{:?} ({})" , & self . inner . kind , position) } else { fmt :: Debug :: fmt (& self . inner . kind , f) } } }
};
}
