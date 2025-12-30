// Generated macro for tests (module)
macro_rules! Depcrate_ext_impls_impl_serdetests {
() => {
// Module: crate::ext_impls::impl_serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_serialize () { let array = GenericArray :: < u8 , typenum :: U2 > :: default () ; let serialized = bincode :: serialize (& array) ; assert ! (serialized . is_ok ()) ; } # [test] fn test_deserialize () { let mut array = GenericArray :: < u8 , typenum :: U2 > :: default () ; array [0] = 1 ; array [1] = 2 ; let serialized = bincode :: serialize (& array) . unwrap () ; let deserialized = bincode :: deserialize :: < GenericArray < u8 , typenum :: U2 > > (& serialized) ; assert ! (deserialized . is_ok ()) ; let array = deserialized . unwrap () ; assert_eq ! (array [0] , 1) ; assert_eq ! (array [1] , 2) ; } # [test] fn test_serialized_size () { let array = GenericArray :: < u8 , typenum :: U1 > :: default () ; let size = bincode :: serialized_size (& array) . unwrap () ; assert_eq ! (size , 1) ; } # [test] # [should_panic] fn test_too_many () { let serialized = "[1, 2, 3, 4, 5]" ; let _ = serde_json :: from_str :: < GenericArray < u8 , typenum :: U4 > > (serialized) . unwrap () ; } }
};
}
