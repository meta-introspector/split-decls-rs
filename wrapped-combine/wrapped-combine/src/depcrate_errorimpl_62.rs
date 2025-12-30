// Generated macro for impl_62 (impl)
macro_rules! Depcrate_errorimpl_62 {
() => {
// Module: crate::error
// Provides: {"impl_62"}
// Dependencies: {}
impl < Item , Range , Position > ParseErrorInto < Item , Range , Position > for UnexpectedParse where Position : Default , { fn into_other_error < T , Item2 , Range2 , Position2 > (self) -> T where T : ParseError < Item2 , Range2 , Position2 > , Item2 : From < Item > , Range2 : From < Range > , Position2 : From < Position > , { T :: from_error (Position :: default () . into () , StreamErrorInto :: < Item , Range > :: into_other_error (self) ,) } }
};
}
