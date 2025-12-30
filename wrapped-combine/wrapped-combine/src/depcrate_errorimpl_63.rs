// Generated macro for impl_63 (impl)
macro_rules! Depcrate_errorimpl_63 {
() => {
// Module: crate::error
// Provides: {"impl_63"}
// Dependencies: {}
impl < Item , Range > StreamErrorInto < Item , Range > for UnexpectedParse { fn into_other_error < T , Item2 , Range2 > (self) -> T where T : StreamError < Item2 , Range2 > , Item2 : From < Item > , Range2 : From < Range > , { StreamError :: into_other (self) } }
};
}
