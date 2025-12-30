// Generated macro for impl_71 (impl)
macro_rules! Depcrate_mapref_entry_refimpl_71 {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a , 'q , K : Eq + Hash , Q , V > EntryRef < 'a , 'q , K , Q , V > { # [doc = " Apply a function to the stored value if it exists."] pub fn and_modify (self , f : impl FnOnce (& mut V)) -> Self { match self { EntryRef :: Occupied (mut entry) => { f (entry . get_mut ()) ; EntryRef :: Occupied (entry) } EntryRef :: Vacant (entry) => EntryRef :: Vacant (entry) , } } }
};
}
