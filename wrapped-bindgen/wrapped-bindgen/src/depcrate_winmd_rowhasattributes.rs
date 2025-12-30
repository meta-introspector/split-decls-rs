// Generated macro for HasAttributes (trait)
macro_rules! Depcrate_winmd_rowHasAttributes {
() => {
// Module: crate::winmd::row
// Provides: {"HasAttributes"}
// Dependencies: {}
pub trait HasAttributes { fn attributes (& self) -> RowIterator < Attribute > ; fn find_attribute (& self , name : & str) -> Option < Attribute > ; fn has_attribute (& self , name : & str) -> bool ; fn guid_attribute (& self) -> Option < GUID > ; fn arches (& self) -> i32 ; }
};
}
