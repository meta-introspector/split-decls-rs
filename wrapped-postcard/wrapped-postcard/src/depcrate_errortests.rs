// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Error ; use core :: fmt :: { self , Display , Formatter } ; use serde :: { Deserialize as _ , Serialize as _ } ; struct DisplayEnumUsingSerde (Error) ; impl Display for DisplayEnumUsingSerde { fn fmt (& self , formatter : & mut Formatter) -> fmt :: Result { Error :: serialize (& self . 0 , formatter) } } # [test] fn test_serde () { for i in 0 .. { let de = serde :: de :: value :: U32Deserializer :: < Error > :: new (i) ; let Ok (error) = Error :: deserialize (de) else { assert_eq ! (i , super :: VARIANT_NAMES . len () as u32) ; break ; } ; assert_eq ! (i , error . clone () as u32) ; let mut buf = [0u8 ; 1] ; crate :: to_slice (& error , & mut buf) . unwrap () ; assert_eq ! (i , buf [0] as u32) ; let string = DisplayEnumUsingSerde (error . clone ()) . to_string () ; assert_eq ! (string , format ! ("{error:?}")) ; let de = serde :: de :: value :: StrDeserializer :: < Error > :: new (& string) ; let error2 = Error :: deserialize (de) . unwrap () ; assert_eq ! (error , error2) ; } } }
};
}
