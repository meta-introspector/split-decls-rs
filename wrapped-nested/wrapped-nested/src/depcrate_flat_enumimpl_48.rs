// Generated macro for impl_48 (impl)
macro_rules! Depcrate_flat_enumimpl_48 {
() => {
// Module: crate::flat_enum
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'sval , S : StreamEnum < 'sval > > FlatStreamEnum < S > { pub fn new (stream : S) -> Self { FlatStreamEnum { stream , queue : Default :: default () , } } pub fn push (& mut self , tag : Option < sval :: Tag > , label : Option < sval :: Label > , index : Option < sval :: Index > ,) -> Result { self . queue . push_back (NestedVariant { tag , label : if let Some (label) = label { Some (owned_label (label) ?) } else { None } , index , }) } pub fn end (self) -> Result < S :: Ok > { self . value_or_recurse (| stream , _ | stream . end () , | stream , _ | stream . end () , ()) } fn value_or_recurse < I > (mut self , value : impl FnOnce (Self , I) -> Result < S :: Ok > , nested : impl FnOnce (FlatStreamEnum < S :: Nested > , I ,) -> Result < < S :: Nested as StreamEnum < 'sval > > :: Ok > , input : I ,) -> Result < S :: Ok > { if let Some (variant) = self . queue . pop_front () { self . stream . nested (variant . tag , variant . label , variant . index , | variant | { nested (FlatStreamEnum { stream : variant , queue : self . queue , } , input ,) }) } else { value (self , input) } } }
};
}
