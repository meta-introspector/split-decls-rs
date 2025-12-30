// Generated macro for StreamSeq (trait)
macro_rules! DepcrateStreamSeq {
() => {
// Module: crate
// Provides: {"StreamSeq"}
// Dependencies: {}
# [doc = "\nA stream for a sequence.\n"] pub trait StreamSeq < 'sval > { # [doc = "\n    The type of value produced by this stream on completion.\n    "] type Ok ; # [doc = "\n    Stream a sequence element.\n    "] fn value < V : sval_ref :: ValueRef < 'sval > > (& mut self , value : V) -> Result { default_stream :: seq_value (self , value) } # [doc = "\n    Stream a reference to a sequence element.\n    "] fn value_ref < V : sval :: Value + ? Sized > (& mut self , value : & 'sval V) -> Result { default_stream :: seq_value_ref (self , value) } # [doc = "\n    Stream a sequence element, borrowed for an arbitrarily short lifetime.\n    "] fn value_computed < V : sval :: Value > (& mut self , value : V) -> Result ; # [doc = "\n    Complete the sequence.\n    "] fn end (self) -> Result < Self :: Ok > ; }
};
}
