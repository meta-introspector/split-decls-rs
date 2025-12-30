// Generated macro for impl_505 (impl)
macro_rules! Depcrate_type_impl_505 {
() => {
// Module: crate::type_
// Provides: {"impl_505"}
// Dependencies: {}
impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { pub fn type_padding_filler (& self , size : Size , align : Align) -> Type < 'gcc > { let unit = Integer :: approximate_align (self , align) ; let size = size . bytes () ; let unit_size = unit . size () . bytes () ; assert_eq ! (size % unit_size , 0) ; self . type_array (self . type_from_integer (unit) , size / unit_size) } pub fn set_struct_body (& self , typ : Struct < 'gcc > , fields : & [Type < 'gcc >] , packed : bool) { let fields : Vec < _ > = fields . iter () . enumerate () . map (| (index , field) | self . context . new_field (None , * field , format ! ("field_{}" , index))) . collect () ; typ . set_fields (None , & fields) ; if packed { # [cfg (feature = "master")] typ . as_type () . set_packed () ; } } pub fn type_named_struct (& self , name : & str) -> Struct < 'gcc > { self . context . new_opaque_struct_type (None , name) } }
};
}
