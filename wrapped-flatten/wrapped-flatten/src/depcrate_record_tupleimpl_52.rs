// Generated macro for impl_52 (impl)
macro_rules! Depcrate_record_tupleimpl_52 {
() => {
// Module: crate::record_tuple
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > Flatten < 'sval > for RecordTupleFlatten < 'sval , S > { type Stream = S ; type LabelStream = LabelBuf < 'sval > ; fn stream (& mut self) -> & mut Self :: Stream { & mut self . stream } fn label_stream (& mut self) -> & mut Self :: LabelStream { & mut self . label_stream } fn flattened_value_begin (& mut self , tag : Option < & Tag > , label : & Label , index : & Index ,) -> sval :: Result { self . stream . record_tuple_value_begin (tag , label , index) } fn flattened_value_end (& mut self , tag : Option < & Tag > , label : & Label , index : & Index ,) -> sval :: Result { self . stream . record_tuple_value_end (tag , label , index) } }
};
}
