// Generated macro for impl_167 (impl)
macro_rules! Depcrate_hpack_headerimpl_167 {
() => {
// Module: crate::hpack::header
// Provides: {"impl_167"}
// Dependencies: {}
impl From < Header > for Header < Option < HeaderName > > { fn from (src : Header) -> Self { match src { Header :: Field { name , value } => Header :: Field { name : Some (name) , value , } , Header :: Authority (v) => Header :: Authority (v) , Header :: Method (v) => Header :: Method (v) , Header :: Scheme (v) => Header :: Scheme (v) , Header :: Path (v) => Header :: Path (v) , Header :: Protocol (v) => Header :: Protocol (v) , Header :: Status (v) => Header :: Status (v) , } } }
};
}
