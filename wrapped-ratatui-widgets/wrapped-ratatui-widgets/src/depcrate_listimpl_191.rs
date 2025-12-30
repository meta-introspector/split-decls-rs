// Generated macro for impl_191 (impl)
macro_rules! Depcrate_listimpl_191 {
() => {
// Module: crate::list
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a , Item > FromIterator < Item > for List < 'a > where Item : Into < ListItem < 'a > > , { fn from_iter < Iter : IntoIterator < Item = Item > > (iter : Iter) -> Self { Self :: new (iter) } }
};
}
