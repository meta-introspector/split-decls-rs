// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a > SectionEntry < 'a > { # [doc = " Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry."] pub fn or_insert (self , properties : Properties) -> & 'a mut Properties { match self { SectionEntry :: Occupied (e) => e . into_mut () , SectionEntry :: Vacant (e) => e . insert (properties) , } } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> Properties > (self , default : F) -> & 'a mut Properties { match self { SectionEntry :: Occupied (e) => e . into_mut () , SectionEntry :: Vacant (e) => e . insert (default ()) , } } }
};
}
