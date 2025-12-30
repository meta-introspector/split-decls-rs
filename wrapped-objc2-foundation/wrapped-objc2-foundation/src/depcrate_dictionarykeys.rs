// Generated macro for Keys (struct)
macro_rules! Depcrate_dictionaryKeys {
() => {
// Module: crate::dictionary
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over the keys of a dictionary."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct Keys < 'a , KeyType : Message , ObjectType : Message > (iter :: Iter < 'a , NSDictionary < KeyType , ObjectType > > ,) ;
};
}
