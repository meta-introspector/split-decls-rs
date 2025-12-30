// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1047 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1047"}
// Dependencies: {}
impl < 'a , 'b , F > Deref for InsertableToReplaceableAdapter < 'a , 'b , F > where F : FnMut (usize) , { type Target = Insertable < 'a , 'b > ; fn deref (& self) -> & Self :: Target { self . child . deref () } }
};
}
