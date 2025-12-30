// Generated macro for impl_188 (impl)
macro_rules! Depcrate_errorimpl_188 {
() => {
// Module: crate::error
// Provides: {"impl_188"}
// Dependencies: {}
impl fmt :: Display for EventKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { EventKind :: StartArray => "StartArray" , EventKind :: StartDictionary => "StartDictionary" , EventKind :: EndCollection => "EndCollection" , EventKind :: Boolean => "Boolean" , EventKind :: Data => "Data" , EventKind :: Date => "Date" , EventKind :: Integer => "Integer" , EventKind :: Real => "Real" , EventKind :: String => "String" , EventKind :: Uid => "Uid" , EventKind :: ValueOrStartCollection => "value or start collection" , EventKind :: DictionaryKeyOrEndCollection => "dictionary key or end collection" , } . fmt (f) } }
};
}
