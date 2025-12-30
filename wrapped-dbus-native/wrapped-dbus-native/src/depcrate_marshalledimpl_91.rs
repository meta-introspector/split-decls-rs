// Generated macro for impl_91 (impl)
macro_rules! Depcrate_marshalledimpl_91 {
() => {
// Module: crate::marshalled
// Provides: {"impl_91"}
// Dependencies: {}
impl DictBuf { pub fn new (key_sig : SignatureSingleBuf , value_sig : SignatureSingleBuf) -> Result < Self , DemarshalError > { let mut x = String :: with_capacity (key_sig . len () + value_sig . len () + 3) ; x . push_str ("a{") ; x . push_str (& key_sig) ; x . push_str (& value_sig) ; x . push_str ("}") ; let x = SignatureSingle :: new_owned (x) . map_err (| _ | DemarshalError :: InvalidString) ? ; Ok (DictBuf { key_sig , value_sig , outer_sig : x , data : vec ! () }) } pub fn append < K : Marshal + ? Sized , V : Marshal + ? Sized > (& mut self , key : & K , value : & V) -> Result < () , DemarshalError > { if & * self . value_sig != value . signature () { return Err (DemarshalError :: WrongType) ; } if & * self . key_sig != key . signature () { return Err (DemarshalError :: WrongType) ; } let old_len = self . data . len () ; align_buf (& mut self . data , 8) ; key . append_data_to (& mut self . data) ; value . append_data_to (& mut self . data) ; if self . data . len () > ARRAY_MAX_LEN { self . data . truncate (old_len) ; Err (DemarshalError :: NumberTooBig) } else { Ok (()) } } }
};
}
