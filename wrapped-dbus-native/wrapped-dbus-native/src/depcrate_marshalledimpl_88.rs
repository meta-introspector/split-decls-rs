// Generated macro for impl_88 (impl)
macro_rules! Depcrate_marshalledimpl_88 {
() => {
// Module: crate::marshalled
// Provides: {"impl_88"}
// Dependencies: {}
impl ArrayBuf { pub fn new (sig : & dbus_strings :: SignatureSingle) -> Result < Self , DemarshalError > { let mut x = String :: with_capacity (sig . len () + 1) ; x . push_str ("a") ; x . push_str (sig) ; let x = SignatureSingle :: new_owned (x) . map_err (| _ | DemarshalError :: InvalidString) ? ; Ok (ArrayBuf { outer_sig : x , data : vec ! () }) } fn verify_array_size (& mut self , old_len : usize) -> Result < () , DemarshalError > { if self . data . len () > ARRAY_MAX_LEN { self . data . truncate (old_len) ; Err (DemarshalError :: NumberTooBig) } else { Ok (()) } } pub fn append < T : Marshal + ? Sized > (& mut self , value : & T) -> Result < () , DemarshalError > { if & self . outer_sig [1 ..] != & * * value . signature () { return Err (DemarshalError :: WrongType) ; } let old_len = self . data . len () ; value . append_data_to (& mut self . data) ; self . verify_array_size (old_len) } pub fn from_iter < 'a , T , I > (iter : I) -> Result < Self , DemarshalError > where T : Marshal + ? Sized + 'a , & 'a T : Default , I : IntoIterator < Item = & 'a T > { let def = < & T > :: default () ; let defsig = def . signature () ; let mut r = ArrayBuf :: new (defsig) ? ; for x in iter . into_iter () { if x . signature () != defsig { return Err (DemarshalError :: WrongType) ; } x . append_data_to (& mut r . data) ; } r . verify_array_size (0) ? ; Ok (r) } }
};
}
