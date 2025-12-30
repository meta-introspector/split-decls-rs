// Generated macro for HasAttributes (trait)
macro_rules! Depcrate_reader_rowHasAttributes {
() => {
// Module: crate::reader::row
// Provides: {"HasAttributes"}
// Dependencies: {}
pub trait HasAttributes < 'a > { fn attributes (& self) -> RowIterator < 'a , Attribute < 'a > > ; fn find_attribute (& self , name : & str) -> Option < Attribute < 'a > > ; fn has_attribute (& self , name : & str) -> bool ; }
};
}
