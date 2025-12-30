// Generated macro for ValueMut (struct)
macro_rules! Depcrate_file_mutable_valueValueMut {
() => {
// Module: crate::file::mutable::value
// Provides: {"ValueMut"}
// Dependencies: {}
# [doc = " An intermediate representation of a mutable value obtained from a [`File`][crate::File]."] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Debug)] pub struct ValueMut < 'borrow , 'lookup , 'event > { pub (crate) section : SectionMut < 'borrow , 'event > , pub (crate) key : section :: ValueName < 'lookup > , pub (crate) index : Index , pub (crate) size : Size , }
};
}
