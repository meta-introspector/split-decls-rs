// Generated macro for Flatten (trait)
macro_rules! Depcrate_flattenerFlatten {
() => {
// Module: crate::flattener
// Provides: {"Flatten"}
// Dependencies: {}
pub (crate) trait Flatten < 'sval > { type Stream : Stream < 'sval > ; type LabelStream : LabelStream < 'sval > ; fn stream (& mut self) -> & mut Self :: Stream ; fn label_stream (& mut self) -> & mut Self :: LabelStream ; fn flattened_value_begin (& mut self , tag : Option < & Tag > , label : & Label , index : & Index ,) -> sval :: Result ; fn flattened_value_end (& mut self , tag : Option < & Tag > , label : & Label , index : & Index ,) -> sval :: Result ; }
};
}
