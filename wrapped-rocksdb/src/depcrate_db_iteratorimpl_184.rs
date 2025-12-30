// Generated macro for impl_184 (impl)
macro_rules! Depcrate_db_iteratorimpl_184 {
() => {
// Module: crate::db_iterator
// Provides: {"impl_184"}
// Dependencies: {}
impl < 'a , D : DBAccess > Into < DBRawIteratorWithThreadMode < 'a , D > > for DBIteratorWithThreadMode < 'a , D > { fn into (self) -> DBRawIteratorWithThreadMode < 'a , D > { self . raw } }
};
}
