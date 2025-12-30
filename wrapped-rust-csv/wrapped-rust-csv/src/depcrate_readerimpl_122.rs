// Generated macro for impl_122 (impl)
macro_rules! Depcrate_readerimpl_122 {
() => {
// Module: crate::reader
// Provides: {"impl_122"}
// Dependencies: {}
impl ReaderState { # [inline (always)] fn add_record (& mut self , record : & ByteRecord) -> Result < () > { let i = self . cur_pos . record () ; self . cur_pos . set_record (i . checked_add (1) . unwrap ()) ; if ! self . flexible { match self . first_field_count { None => self . first_field_count = Some (record . len () as u64) , Some (expected) => { if record . len () as u64 != expected { return Err (Error :: new (ErrorKind :: UnequalLengths { pos : record . position () . cloned () , expected_len : expected , len : record . len () as u64 , })) ; } } } } Ok (()) } }
};
}
