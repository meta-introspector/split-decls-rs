// Generated macro for Drain (struct)
macro_rules! Depcrate_header_mapDrain {
() => {
// Module: crate::header::map
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A drain iterator for `HeaderMap`."] # [derive (Debug)] pub struct Drain < 'a , T > { idx : usize , len : usize , entries : * mut [Bucket < T >] , next : Option < usize > , extra_values : * mut Vec < ExtraValue < T > > , lt : PhantomData < & 'a mut HeaderMap < T > > , }
};
}
