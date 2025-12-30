// Generated macro for impl_46 (impl)
macro_rules! Depcrate_recordimpl_46 {
() => {
// Module: crate::record
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > Flatten < 'sval > for RecordFlatten < 'sval , S > { type Stream = S ; type LabelStream = LabelBuf < 'sval > ; fn stream (& mut self) -> & mut Self :: Stream { & mut self . stream } fn label_stream (& mut self) -> & mut Self :: LabelStream { & mut self . label_stream } fn flattened_value_begin (& mut self , tag : Option < & Tag > , label : & Label , _ : & Index ,) -> sval :: Result { self . stream . record_value_begin (tag , label) } fn flattened_value_end (& mut self , tag : Option < & Tag > , label : & Label , _ : & Index) -> sval :: Result { self . stream . record_value_end (tag , label) } }
};
}
