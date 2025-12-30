// Generated macro for impl_34 (impl)
macro_rules! Depcrate_to_fmtimpl_34 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'sval , W > Formatter < W > where W : Write , { fn internally_tagged_begin (& mut self , label : Option < & sval :: Label > , index : Option < & sval :: Index > ,) -> sval :: Result { if self . is_internally_tagged { self . is_internally_tagged = false ; if let Some (label) = label { return self . internally_tagged_map_begin_label (label . as_str ()) ; } else if let Some (index) = index . and_then (| index | index . to_i64 ()) { return self . internally_tagged_map_begin_index (index) ; } } Ok (()) } fn internally_tagged_end (& mut self , label : Option < & sval :: Label > , index : Option < & sval :: Index > ,) -> sval :: Result { self . is_internally_tagged = label . is_some () || index . and_then (| index | index . to_i64 ()) . is_some () ; Ok (()) } fn internally_tagged_map_begin_label (& mut self , label : & str) -> sval :: Result { _try_no_conv ! (self . map_begin (Some (1))) ; _try_no_conv ! (self . map_key_begin ()) ; _try ! (escape_str (label , & mut self . out)) ; _try_no_conv ! (self . map_key_end ()) ; self . map_value_begin () } fn internally_tagged_map_begin_index (& mut self , index : i64) -> sval :: Result { _try_no_conv ! (self . map_begin (Some (1))) ; _try_no_conv ! (self . map_key_begin ()) ; _try_no_conv ! (self . i64 (index)) ; _try_no_conv ! (self . map_key_end ()) ; self . map_value_begin () } fn internally_tagged_map_end (& mut self) -> sval :: Result { _try_no_conv ! (self . map_value_end ()) ; self . map_end () } }
};
}
