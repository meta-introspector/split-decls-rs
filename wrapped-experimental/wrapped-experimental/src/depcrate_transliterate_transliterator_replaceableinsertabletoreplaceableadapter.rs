// Generated macro for InsertableToReplaceableAdapter (struct)
macro_rules! Depcrate_transliterate_transliterator_replaceableInsertableToReplaceableAdapter {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"InsertableToReplaceableAdapter"}
// Dependencies: {}
# [doc = " See [`Insertable::start_replaceable_adapter`]."] pub (super) struct InsertableToReplaceableAdapter < 'a , 'b , F > where F : FnMut (usize) , { child : ManuallyDrop < Insertable < 'a , 'b > > , range_start : usize , on_drop : F , }
};
}
