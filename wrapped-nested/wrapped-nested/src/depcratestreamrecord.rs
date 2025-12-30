// Generated macro for StreamRecord (trait)
macro_rules! DepcrateStreamRecord {
() => {
// Module: crate
// Provides: {"StreamRecord"}
// Dependencies: {}
# [doc = "\nA stream for a record.\n"] pub trait StreamRecord < 'sval > { # [doc = "\n    The type of value produced by this stream on completion.\n    "] type Ok ; # [doc = "\n    Stream a record field.\n    "] fn value < V : sval_ref :: ValueRef < 'sval > > (& mut self , tag : Option < sval :: Tag > , label : sval :: Label , value : V ,) -> Result { default_stream :: record_value (self , tag , label , value) } # [doc = "\n    Stream a reference to a record field.\n    "] fn value_ref < V : sval :: Value + ? Sized > (& mut self , tag : Option < sval :: Tag > , label : sval :: Label , value : & 'sval V ,) -> Result { default_stream :: record_value_ref (self , tag , label , value) } # [doc = "\n    Stream a record field, borrowed for an arbitrarily short lifetime.\n    "] fn value_computed < V : sval :: Value > (& mut self , tag : Option < sval :: Tag > , label : sval :: Label , value : V ,) -> Result ; # [doc = "\n    Complete the record.\n    "] fn end (self) -> Result < Self :: Ok > ; }
};
}
