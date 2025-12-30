// Generated macro for impl_35 (impl)
macro_rules! Depcrate_mapimpl_35 {
() => {
// Module: crate::map
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > Flatten < 'sval > for MapFlatten < S > { type Stream = PassThru < S > ; type LabelStream = PassThru < S > ; fn stream (& mut self) -> & mut Self :: Stream { & mut self . stream } fn label_stream (& mut self) -> & mut Self :: LabelStream { & mut self . stream } fn flattened_value_begin (& mut self , _ : Option < & Tag > , label : & Label , _ : & Index) -> sval :: Result { if ! self . stream . seen_key { self . stream . map_key_begin () ? ; if let Some (label) = label . as_static_str () { self . stream . value (label) ? ; } else { self . stream . value_computed (label . as_str ()) ? ; } self . stream . map_key_end () ? ; } self . stream . map_value_begin () } fn flattened_value_end (& mut self , _ : Option < & Tag > , _ : & Label , _ : & Index) -> sval :: Result { self . stream . map_value_end () } }
};
}
