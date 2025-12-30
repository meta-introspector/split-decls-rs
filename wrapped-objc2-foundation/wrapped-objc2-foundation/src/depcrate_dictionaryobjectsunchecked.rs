// Generated macro for ObjectsUnchecked (struct)
macro_rules! Depcrate_dictionaryObjectsUnchecked {
() => {
// Module: crate::dictionary
// Provides: {"ObjectsUnchecked"}
// Dependencies: {}
# [doc = " An iterator over unretained objects / values of a dictionary."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The dictionary must not be mutated while this is alive."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct ObjectsUnchecked < 'a , KeyType : Message , ObjectType : Message + 'a > (iter :: IterUncheckedWithBackingEnum < 'a , NSDictionary < KeyType , ObjectType > , crate :: NSEnumerator < ObjectType > , > ,) ;
};
}
