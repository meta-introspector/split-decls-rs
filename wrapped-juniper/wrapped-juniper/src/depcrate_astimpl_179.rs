// Generated macro for impl_179 (impl)
macro_rules! Depcrate_astimpl_179 {
() => {
// Module: crate::ast
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , S > Arguments < 'a , S > { pub fn into_iter (self) -> vec :: IntoIter < (Spanning < & 'a str > , Spanning < InputValue < S > >) > { self . items . into_iter () } pub fn iter (& self) -> slice :: Iter < '_ , (Spanning < & 'a str > , Spanning < InputValue < S > >) > { self . items . iter () } pub fn iter_mut (& mut self) -> slice :: IterMut < '_ , (Spanning < & 'a str > , Spanning < InputValue < S > >) > { self . items . iter_mut () } pub fn len (& self) -> usize { self . items . len () } pub fn get (& self , key : & str) -> Option < & Spanning < InputValue < S > > > { self . items . iter () . filter (| & (k , _) | k . item == key) . map (| (_ , v) | v) . next () } }
};
}
