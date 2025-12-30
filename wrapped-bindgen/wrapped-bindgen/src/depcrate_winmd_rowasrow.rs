// Generated macro for AsRow (trait)
macro_rules! Depcrate_winmd_rowAsRow {
() => {
// Module: crate::winmd::row
// Provides: {"AsRow"}
// Dependencies: {}
pub trait AsRow : Copy { const TABLE : usize ; fn to_row (& self) -> Row ; fn from_row (row : Row) -> Self ; fn file (& self) -> & 'static File { unsafe { & * self . to_row () . file } } fn reader (& self) -> & 'static Reader { unsafe { & * self . file () . reader } } fn index (& self) -> usize { self . to_row () . index } fn usize (& self , column : usize) -> usize { self . file () . usize (self . index () , Self :: TABLE , column) } fn str (& self , column : usize) -> & 'static str { self . file () . str (self . index () , Self :: TABLE , column) } fn row (& self , column : usize) -> Row { Row :: new (self . file () , self . usize (column) - 1) } fn decode < T : Decode > (& self , column : usize) -> T { T :: decode (self . file () , self . usize (column)) } fn blob (& self , column : usize) -> Blob { self . file () . blob (self . index () , Self :: TABLE , column) } fn list < R : AsRow > (& self , column : usize) -> RowIterator < R > { self . file () . list (self . index () , Self :: TABLE , column) } }
};
}
