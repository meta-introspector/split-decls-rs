// Generated macro for ValueDrain (struct)
macro_rules! Depcrate_header_mapValueDrain {
() => {
// Module: crate::header::map
// Provides: {"ValueDrain"}
// Dependencies: {}
# [doc = " An drain iterator of all values associated with a single header name."] # [derive (Debug)] pub struct ValueDrain < 'a , T > { first : Option < T > , next : Option < :: std :: vec :: IntoIter < T > > , lt : PhantomData < & 'a mut HeaderMap < T > > , }
};
}
