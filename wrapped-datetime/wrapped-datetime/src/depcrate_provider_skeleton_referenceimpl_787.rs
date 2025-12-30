// Generated macro for impl_787 (impl)
macro_rules! Depcrate_provider_skeleton_referenceimpl_787 {
() => {
// Module: crate::provider::skeleton::reference
// Provides: {"impl_787"}
// Dependencies: {}
# [doc = " Parse a string into a list of fields. This trait implementation validates the input string to"] # [doc = " verify that fields are correct. If the fields are out of order, this returns an error that"] # [doc = " contains the fields, which gives the callee a chance to sort the fields with the"] # [doc = " `From<SmallVec<[fields::Field; 5]>> for Skeleton` trait."] impl TryFrom < & str > for Skeleton { type Error = SkeletonError ; fn try_from (skeleton_string : & str) -> Result < Self , Self :: Error > { let mut fields : SmallVec < [fields :: Field ; 5] > = SmallVec :: new () ; let mut iter = skeleton_string . chars () . peekable () ; while let Some (ch) = iter . next () { let mut field_length : u8 = 1 ; while let Some (next_ch) = iter . peek () { if * next_ch != ch { break ; } field_length += 1 ; iter . next () ; } let field_symbol = if ch == 'Z' { match field_length { 1 ..= 3 => { field_length = 4 ; FieldSymbol :: try_from ('x') ? } 4 => FieldSymbol :: try_from ('O') ? , 5 => { field_length = 4 ; FieldSymbol :: try_from ('X') ? } _ => FieldSymbol :: try_from (ch) ? , } } else { FieldSymbol :: try_from (ch) ? } ; let field = Field :: from ((field_symbol , FieldLength :: from_idx (field_length) ?)) ; match fields . binary_search (& field) { Ok (_) => return Err (SkeletonError :: DuplicateField) , Err (pos) => fields . insert (pos , field) , } } Ok (Self :: from (fields)) } }
};
}
