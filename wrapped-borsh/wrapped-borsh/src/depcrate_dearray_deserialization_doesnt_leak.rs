// Generated macro for array_deserialization_doesnt_leak (function)
macro_rules! Depcrate_dearray_deserialization_doesnt_leak {
() => {
// Module: crate::de
// Provides: {"array_deserialization_doesnt_leak"}
// Dependencies: {}
# [test] fn array_deserialization_doesnt_leak () { use core :: sync :: atomic :: { AtomicUsize , Ordering } ; static DESERIALIZE_COUNT : AtomicUsize = AtomicUsize :: new (0) ; static DROP_COUNT : AtomicUsize = AtomicUsize :: new (0) ; # [allow (unused)] struct MyType (u8) ; impl BorshDeserialize for MyType { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let val = u8 :: deserialize_reader (reader) ? ; let v = DESERIALIZE_COUNT . fetch_add (1 , Ordering :: SeqCst) ; if v >= 7 { panic ! ("panic in deserialize") ; } Ok (MyType (val)) } } impl Drop for MyType { fn drop (& mut self) { DROP_COUNT . fetch_add (1 , Ordering :: SeqCst) ; } } assert ! (< [MyType ; 5] as BorshDeserialize >:: deserialize (& mut & [0u8 ; 3] [..]) . is_err ()) ; assert_eq ! (DESERIALIZE_COUNT . load (Ordering :: SeqCst) , 3) ; assert_eq ! (DROP_COUNT . load (Ordering :: SeqCst) , 3) ; assert ! (< [MyType ; 2] as BorshDeserialize >:: deserialize (& mut & [0u8 ; 2] [..]) . is_ok ()) ; assert_eq ! (DESERIALIZE_COUNT . load (Ordering :: SeqCst) , 5) ; assert_eq ! (DROP_COUNT . load (Ordering :: SeqCst) , 5) ; # [cfg (feature = "std")] { let result = std :: panic :: catch_unwind (| | { < [MyType ; 3] as BorshDeserialize > :: deserialize (& mut & [0u8 ; 3] [..]) . unwrap () ; }) ; assert ! (result . is_err ()) ; assert_eq ! (DESERIALIZE_COUNT . load (Ordering :: SeqCst) , 8) ; assert_eq ! (DROP_COUNT . load (Ordering :: SeqCst) , 7) ; } }
};
}
