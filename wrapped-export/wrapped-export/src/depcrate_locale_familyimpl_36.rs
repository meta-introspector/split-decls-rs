// Generated macro for impl_36 (impl)
macro_rules! Depcrate_locale_familyimpl_36 {
() => {
// Module: crate::locale_family
// Provides: {"impl_36"}
// Dependencies: {}
impl Writeable for DataLocaleFamilyAnnotations { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { match (self . include_ancestors , self . include_descendants) { (true , true) => Ok (()) , (true , false) => sink . write_char ('^') , (false , true) => sink . write_char ('%') , (false , false) => sink . write_char ('@') , } } fn writeable_length_hint (& self) -> writeable :: LengthHint { writeable :: LengthHint :: exact (match (self . include_ancestors , self . include_descendants) { (true , true) => 0 , _ => 1 , }) } }
};
}
