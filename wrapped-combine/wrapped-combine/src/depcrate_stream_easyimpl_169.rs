// Generated macro for impl_169 (impl)
macro_rules! Depcrate_stream_easyimpl_169 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_169"}
// Dependencies: {}
impl < Item , Range , Position > crate :: error :: ParseErrorInto < Item , Range , Position > for Errors < Item , Range , Position > { fn into_other_error < T , Item2 , Range2 , Position2 > (self) -> T where T : crate :: error :: ParseError < Item2 , Range2 , Position2 > , Item2 : From < Item > , Range2 : From < Range > , Position2 : From < Position > , { let mut error = T :: empty (self . position . into ()) ; for err in self . errors { error . add (crate :: error :: StreamErrorInto :: < Item , Range > :: into_other_error (err)) ; } error } }
};
}
