// Generated macro for StreamMap (trait)
macro_rules! DepcrateStreamMap {
() => {
// Module: crate
// Provides: {"StreamMap"}
// Dependencies: {}
# [doc = "\nA stream for a map.\n"] pub trait StreamMap < 'sval > { # [doc = "\n    The type of value produced by this stream on completion.\n    "] type Ok ; # [doc = "\n    Stream a map key.\n    "] fn key < V : sval_ref :: ValueRef < 'sval > > (& mut self , key : V) -> Result { default_stream :: map_key (self , key) } # [doc = "\n    Stream a reference to a map key.\n    "] fn key_ref < V : sval :: Value + ? Sized > (& mut self , key : & 'sval V) -> Result { default_stream :: map_key_ref (self , key) } # [doc = "\n    Stream a map key, borrowed for an arbitrarily short lifetime.\n    "] fn key_computed < V : sval :: Value > (& mut self , key : V) -> Result ; # [doc = "\n    Stream a map value.\n    "] fn value < V : sval_ref :: ValueRef < 'sval > > (& mut self , value : V) -> Result { default_stream :: map_value (self , value) } # [doc = "\n    Stream a reference to a map value.\n    "] fn value_ref < V : sval :: Value + ? Sized > (& mut self , value : & 'sval V) -> Result { default_stream :: map_value_ref (self , value) } # [doc = "\n    Stream a map value, borrowed for an arbitrarily short lifetime.\n    "] fn value_computed < V : sval :: Value > (& mut self , value : V) -> Result ; # [doc = "\n    Complete the map.\n    "] fn end (self) -> Result < Self :: Ok > ; }
};
}
