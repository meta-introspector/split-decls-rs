// Generated macro for impl_90 (impl)
macro_rules! Depcrate_look_aheadimpl_90 {
() => {
// Module: crate::look_ahead
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > Lookahead < 'a > { pub (crate) fn new (fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , field : & 'a Field , context : & 'a Context < 'a > ,) -> Self { Self { fragments , fields : vec ! [field] , context , } } # [doc = " Get the field of the selection set with the specified name. This will"] # [doc = " ignore aliases."] # [doc = ""] # [doc = " For example, calling `.field(\"a\")` on `{ a { b } }` will return a"] # [doc = " lookahead that represents `{ b }`."] # [must_use] pub fn field (& self , name : & str) -> Self { let mut fields = Vec :: new () ; for field in & self . fields { filter (& mut fields , self . fragments , & field . selection_set . node , name) } Self { fragments : self . fragments , fields , context : self . context , } } # [doc = " Returns true if field exists otherwise return false."] # [inline] pub fn exists (& self) -> bool { ! self . fields . is_empty () } # [doc = " Get the `SelectionField`s for each of the fields covered by this"] # [doc = " `Lookahead`."] # [doc = ""] # [doc = " There will be multiple fields in situations where the same field is"] # [doc = " queried twice."] pub fn selection_fields (& self) -> Vec < SelectionField < 'a > > { self . fields . iter () . map (| field | SelectionField { fragments : self . fragments , field , context : self . context , }) . collect () } }
};
}
