// Generated macro for impl_72 (impl)
macro_rules! Depcrate_errorimpl_72 {
() => {
// Module: crate::error
// Provides: {"impl_72"}
// Dependencies: {}
impl < Item , Range > StreamErrorInto < Item , Range > for StringStreamError { fn into_other_error < T , Item2 , Range2 > (self) -> T where T : StreamError < Item2 , Range2 > , Item2 : From < Item > , Range2 : From < Range > , { StreamError :: into_other (self) } }
};
}
