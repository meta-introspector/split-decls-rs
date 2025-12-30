// Generated macro for KeysUnchecked (struct)
macro_rules! Depcrate_dictionaryKeysUnchecked {
() => {
// Module: crate::dictionary
// Provides: {"KeysUnchecked"}
// Dependencies: {}
# [doc = " An iterator over unretained keys of a dictionary."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The dictionary must not be mutated while this is alive."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct KeysUnchecked < 'a , KeyType : Message , ObjectType : Message > (iter :: IterUnchecked < 'a , NSDictionary < KeyType , ObjectType > > ,) ;
};
}
