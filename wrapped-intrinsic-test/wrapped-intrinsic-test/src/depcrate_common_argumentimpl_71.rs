// Generated macro for impl_71 (impl)
macro_rules! Depcrate_common_argumentimpl_71 {
() => {
// Module: crate::common::argument
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > Argument < T > where T : IntrinsicTypeDefinition , { pub fn new (pos : usize , name : String , ty : T , constraint : Option < Constraint >) -> Self { Argument { pos , name , ty , constraint , } } pub fn to_c_type (& self) -> String { self . ty . c_type () } pub fn is_simd (& self) -> bool { self . ty . is_simd () } pub fn is_ptr (& self) -> bool { self . ty . is_ptr () } pub fn has_constraint (& self) -> bool { self . constraint . is_some () } # [doc = " The binding keyword (e.g. \"const\" or \"let\") for the array of possible test inputs."] fn rust_vals_array_binding (& self) -> impl std :: fmt :: Display { if self . ty . is_rust_vals_array_const () { "const" } else { "let" } } # [doc = " The name (e.g. \"A_VALS\" or \"a_vals\") for the array of possible test inputs."] fn rust_vals_array_name (& self) -> impl std :: fmt :: Display { if self . ty . is_rust_vals_array_const () { format ! ("{}_VALS" , self . name . to_uppercase ()) } else { format ! ("{}_vals" , self . name . to_lowercase ()) } } fn as_call_param_c (& self) -> String { self . ty . as_call_param_c (& self . name) } }
};
}
