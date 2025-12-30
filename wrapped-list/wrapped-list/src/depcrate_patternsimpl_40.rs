// Generated macro for impl_40 (impl)
macro_rules! Depcrate_patternsimpl_40 {
() => {
// Module: crate::patterns
// Provides: {"impl_40"}
// Dependencies: {}
impl ListFormatterPatterns < '_ > { # [doc = " Creates a new [`ListFormatterPatterns`] from the given patterns. Fails if any pattern is invalid."] # [cfg (feature = "datagen")] pub fn try_new (start : & str , middle : & str , end : & str , pair : & str) -> Result < Self , DataError > { use zerovec :: VarZeroCow ; let err = DataError :: custom ("Invalid list pattern") ; Ok (Self { start : ListJoinerPattern :: try_from_str (start , true , false) ? , middle : VarZeroCow :: new_owned (middle . strip_prefix ("{0}") . ok_or (err) ? . strip_suffix ("{1}") . ok_or (err) ? . to_string () . into_boxed_str () ,) , end : ListJoinerPattern :: try_from_str (end , false , true) ? . into () , pair : if end != pair { Some (ListJoinerPattern :: try_from_str (pair , true , true) ? . into ()) } else { None } , }) } # [doc = " The range of the number of bytes required by the list literals to join a"] # [doc = " list of length `len`. If none of the patterns are conditional, this is exact."] pub (crate) fn length_hint (& self , len : usize) -> LengthHint { match len { 0 | 1 => LengthHint :: exact (0) , 2 => self . pair . as_ref () . unwrap_or (& self . end) . size_hint () , n => { self . start . size_hint () + self . middle . writeable_length_hint () * (n - 3) + self . end . size_hint () } } } }
};
}
