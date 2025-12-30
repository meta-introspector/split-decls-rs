// Generated macro for impl_289 (impl)
macro_rules! Depcrate_read_cfiimpl_289 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_289"}
// Dependencies: {}
impl < T : ReaderOffset > UnwindExpression < T > { # [doc = " Get the expression from the section."] # [doc = ""] # [doc = " The offset and length were previously validated when the"] # [doc = " `UnwindExpression` was created, so this should not fail."] pub fn get < R , S > (& self , section : & S) -> Result < Expression < R > > where R : Reader < Offset = T > , S : UnwindSection < R > , { let input = & mut section . section () . clone () ; input . skip (self . offset) ? ; let data = input . split (self . length) ? ; Ok (Expression (data)) } }
};
}
