// Generated macro for StreamTuple (trait)
macro_rules! DepcrateStreamTuple {
() => {
// Module: crate
// Provides: {"StreamTuple"}
// Dependencies: {}
# [doc = "\nA stream for a tuple.\n"] pub trait StreamTuple < 'sval > { # [doc = "\n    The type of value produced by this stream on completion.\n    "] type Ok ; # [doc = "\n    Stream a tuple field.\n    "] fn value < V : sval_ref :: ValueRef < 'sval > > (& mut self , tag : Option < sval :: Tag > , index : sval :: Index , value : V ,) -> Result { default_stream :: tuple_value (self , tag , index , value) } # [doc = "\n    Stream a reference to a tuple field.\n    "] fn value_ref < V : sval :: Value + ? Sized > (& mut self , tag : Option < sval :: Tag > , index : sval :: Index , value : & 'sval V ,) -> Result { default_stream :: tuple_value_ref (self , tag , index , value) } # [doc = "\n    Stream a tuple field, borrowed for an arbitrarily short lifetime.\n    "] fn value_computed < V : sval :: Value > (& mut self , tag : Option < sval :: Tag > , index : sval :: Index , value : V ,) -> Result ; # [doc = "\n    Complete the tuple.\n    "] fn end (self) -> Result < Self :: Ok > ; }
};
}
