// Generated macro for impl_66 (impl)
macro_rules! Depcrate_tupleimpl_66 {
() => {
// Module: crate::tuple
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > Flatten < 'sval > for TupleFlatten < S > { type Stream = S ; type LabelStream = Empty ; fn stream (& mut self) -> & mut Self :: Stream { & mut self . stream } fn label_stream (& mut self) -> & mut Self :: LabelStream { & mut self . label_stream } fn flattened_value_begin (& mut self , tag : Option < & Tag > , _ : & Label , index : & Index ,) -> sval :: Result { self . stream . tuple_value_begin (tag , index) } fn flattened_value_end (& mut self , tag : Option < & Tag > , _ : & Label , index : & Index) -> sval :: Result { self . stream . tuple_value_end (tag , index) } }
};
}
