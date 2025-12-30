// Generated macro for impl_43 (impl)
macro_rules! Depcrate_patternsimpl_43 {
() => {
// Module: crate::patterns
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'data > ListJoinerPattern < 'data > { # [cfg (feature = "datagen")] # [doc = " Parses a [`ListJoinerPattern`] from a string containing the \"{0}\" and \"{1}\" placeholders."] pub fn try_from_str (pattern : & str , allow_prefix : bool , allow_suffix : bool ,) -> Result < Self , DataError > { match (pattern . find ("{0}") , pattern . find ("{1}")) { (Some (index_0) , Some (index_1)) if index_0 < index_1 && (allow_prefix || index_0 == 0) && (allow_suffix || index_1 == pattern . len () - 3) => { if (index_0 > 0 && ! cfg ! (test)) || index_1 - 3 >= 256 { return Err (DataError :: custom ("Found valid pattern that cannot be stored in ListFormatterPatterns" ,) . with_debug_context (pattern)) ; } Ok (ListJoinerPattern { string : VarZeroCow :: new_owned (alloc :: format ! ("{}{}{}" , & pattern [0 .. index_0] , & pattern [index_0 + 3 .. index_1] , & pattern [index_1 + 3 ..]) . into_boxed_str () ,) , index_0 : index_0 as u8 , index_1 : (index_1 - 3) as u8 , }) } _ => Err (DataError :: custom ("Invalid list pattern") . with_debug_context (pattern)) , } } pub (crate) fn parts (& 'data self) -> PatternParts < 'data > { # ! [allow (clippy :: indexing_slicing)] let index_0 = self . index_0 as usize ; let index_1 = self . index_1 as usize ; (& self . string [0 .. index_0] , & self . string [index_0 .. index_1] , & self . string [index_1 ..] ,) } fn size_hint (& self) -> LengthHint { LengthHint :: exact (self . string . len ()) } }
};
}
