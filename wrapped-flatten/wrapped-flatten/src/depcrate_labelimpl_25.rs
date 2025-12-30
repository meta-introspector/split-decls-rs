// Generated macro for impl_25 (impl)
macro_rules! Depcrate_labelimpl_25 {
() => {
// Module: crate::label
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'sval > Stream < 'sval > for LabelBuf < 'sval > { fn null (& mut self) -> sval :: Result { self . null () } fn bool (& mut self , value : bool) -> sval :: Result { self . bool (value) } fn text_begin (& mut self , _ : Option < usize >) -> sval :: Result { Ok (()) } fn text_fragment (& mut self , fragment : & 'sval str) -> sval :: Result { self . text_fragment (fragment) } fn text_fragment_computed (& mut self , fragment : & str) -> sval :: Result { self . text_fragment_computed (fragment) } fn text_end (& mut self) -> sval :: Result { Ok (()) } fn u64 (& mut self , value : u64) -> sval :: Result { self . u128 (value) } fn u128 (& mut self , value : u128) -> sval :: Result { self . u128 (value) } fn i64 (& mut self , value : i64) -> sval :: Result { self . i128 (value) } fn i128 (& mut self , value : i128) -> sval :: Result { self . i128 (value) } fn f64 (& mut self , value : f64) -> sval :: Result { self . f64 (value) } fn seq_begin (& mut self , _ : Option < usize >) -> sval :: Result { Ok (()) } fn seq_value_begin (& mut self) -> sval :: Result { Ok (()) } fn seq_value_end (& mut self) -> sval :: Result { Ok (()) } fn seq_end (& mut self) -> sval :: Result { Ok (()) } fn tag (& mut self , _ : Option < & Tag > , label : Option < & Label > , index : Option < & Index > ,) -> sval :: Result { if let Some (label) = label { self . label (label) } else if let Some (index) = index { self . index (index) } else { self . null () } } }
};
}
