// Generated macro for impl_191 (impl)
macro_rules! Depcrate_dictionaryimpl_191 {
() => {
// Module: crate::dictionary
// Provides: {"impl_191"}
// Dependencies: {}
# [doc = " Convenience creation methods."] impl < KeyType : Message , ObjectType : Message > NSMutableDictionary < KeyType , ObjectType > { # [cfg (feature = "NSObject")] pub fn from_slices < CopiedKey > (keys : & [& CopiedKey] , objects : & [& ObjectType]) -> Retained < Self > where CopiedKey : Message + NSCopying + CopyingHelper < Result = KeyType > , { assert_eq ! (keys . len () , objects . len () , "key slice and object slice should have the same length" ,) ; let count = keys . len () ; let keys = keys_to_ptr (keys) ; let objects = util :: ref_ptr_cast_const (objects . as_ptr ()) ; unsafe { Self :: initWithObjects_forKeys_count (Self :: alloc () , objects , keys , count) } } # [cfg (feature = "NSObject")] pub fn from_retained_objects < CopiedKey > (keys : & [& CopiedKey] , objects : & [Retained < ObjectType >] ,) -> Retained < Self > where CopiedKey : Message + NSCopying + CopyingHelper < Result = KeyType > , { assert_eq ! (keys . len () , objects . len () , "key slice and object slice should have the same length" ,) ; let count = keys . len () ; let keys = keys_to_ptr (keys) ; let objects = util :: retained_ptr_cast_const (objects . as_ptr ()) ; unsafe { Self :: initWithObjects_forKeys_count (Self :: alloc () , objects , keys , count) } } }
};
}
