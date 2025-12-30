// Generated macro for impl_59 (impl)
macro_rules! Depcrate_seqimpl_59 {
() => {
// Module: crate::seq
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > Flatten < 'sval > for SeqFlatten < S > { type Stream = S ; type LabelStream = Empty ; fn stream (& mut self) -> & mut Self :: Stream { & mut self . stream } fn label_stream (& mut self) -> & mut Self :: LabelStream { & mut self . label_stream } fn flattened_value_begin (& mut self , _ : Option < & Tag > , _ : & Label , _ : & Index) -> sval :: Result { self . stream . seq_value_begin () } fn flattened_value_end (& mut self , _ : Option < & Tag > , _ : & Label , _ : & Index) -> sval :: Result { self . stream . seq_value_end () } }
};
}
