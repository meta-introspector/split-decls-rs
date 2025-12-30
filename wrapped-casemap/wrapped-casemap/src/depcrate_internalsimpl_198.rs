// Generated macro for impl_198 (impl)
macro_rules! Depcrate_internalsimpl_198 {
() => {
// Module: crate::internals
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a , const IS_TITLE_CONTEXT : bool > Writeable for FullCaseWriteable < 'a , '_ , IS_TITLE_CONTEXT > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { let src = self . src ; let mut mapping = self . mapping ; let mut iter = src . char_indices () ; for (i , c) in & mut iter { let context = ContextIterator :: new (& src [.. i] , & src [i ..]) ; self . data . full_helper :: < IS_TITLE_CONTEXT , W > (c , context , self . locale , mapping , sink) ? ; if IS_TITLE_CONTEXT { if self . titlecase_tail_casing == TrailingCase :: Lower { mapping = MappingKind :: Lower ; } else { break ; } } } if IS_TITLE_CONTEXT && self . titlecase_tail_casing == TrailingCase :: Unchanged { sink . write_str (iter . as_str ()) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { writeable :: LengthHint :: at_least (self . src . len ()) } fn write_to_string (& self) -> alloc :: borrow :: Cow < 'a , str > { writeable :: to_string_or_borrow (self , self . src . as_bytes ()) } }
};
}
