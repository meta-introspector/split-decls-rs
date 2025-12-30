// Generated macro for Comdat (struct)
macro_rules! Depcrate_read_anyComdat {
() => {
// Module: crate::read::any
// Provides: {"Comdat"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`File`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] pub struct Comdat < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : ComdatInternal < 'data , 'file , R > , }
};
}
