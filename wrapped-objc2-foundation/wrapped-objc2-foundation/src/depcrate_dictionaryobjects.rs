// Generated macro for Objects (struct)
macro_rules! Depcrate_dictionaryObjects {
() => {
// Module: crate::dictionary
// Provides: {"Objects"}
// Dependencies: {}
# [doc = " An iterator over the objects / values in a dictionary."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct Objects < 'a , KeyType : Message , ObjectType : Message > (iter :: IterWithBackingEnum < 'a , NSDictionary < KeyType , ObjectType > , crate :: NSEnumerator < ObjectType > , > ,) ;
};
}
