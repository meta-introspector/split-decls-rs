// Generated macro for StackItem (enum)
macro_rules! Depcrate_streamStackItem {
() => {
// Module: crate::stream
// Provides: {"StackItem"}
// Dependencies: {}
enum StackItem < 'a > { Root (& 'a Value) , Array (std :: slice :: Iter < 'a , Value >) , Dict (dictionary :: Iter < 'a >) , DictValue (& 'a Value) , }
};
}
