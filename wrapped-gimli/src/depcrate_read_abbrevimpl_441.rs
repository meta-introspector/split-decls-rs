// Generated macro for impl_441 (impl)
macro_rules! Depcrate_read_abbrevimpl_441 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_441"}
// Dependencies: {}
impl Attributes { # [doc = " Returns a new empty list of attributes"] fn new () -> Attributes { let default = AttributeSpecification :: new (constants :: DW_AT_null , constants :: DW_FORM_null , None) ; Attributes :: Inline { buf : [default ; 5] , len : 0 , } } # [doc = " Pushes a new value onto this list of attributes."] fn push (& mut self , attr : AttributeSpecification) { match self { Attributes :: Heap (list) => list . push (attr) , Attributes :: Inline { buf , len : MAX_ATTRIBUTES_INLINE , } => { let mut list = buf . to_vec () ; list . push (attr) ; * self = Attributes :: Heap (list) ; } Attributes :: Inline { buf , len } => { buf [* len] = attr ; * len += 1 ; } } } }
};
}
