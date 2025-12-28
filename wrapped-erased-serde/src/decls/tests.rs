macro_rules! deps {
    () => {
        Serialize!();
        Error!();
        Result!();
        Serializer!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; use alloc :: { vec , vec :: Vec } ; use serde_derive :: Serialize ; fn test_json < T > (t : T) where T : serde :: Serialize , { let expected = serde_json :: to_vec (& t) . unwrap () ; { let obj : & dyn Serialize = & t ; let mut buf = Vec :: new () ; { let mut ser = serde_json :: Serializer :: new (& mut buf) ; let ser : & mut dyn Serializer = & mut < dyn Serializer > :: erase (& mut ser) ; obj . erased_serialize (ser) . unwrap () ; } assert_eq ! (buf , expected) ; } { let obj : Box < dyn Serialize > = Box :: new (t) ; let mut buf = Vec :: new () ; { let mut ser = serde_json :: Serializer :: new (& mut buf) ; let mut ser : Box < dyn Serializer > = Box :: new (< dyn Serializer > :: erase (& mut ser)) ; obj . erased_serialize (& mut ser) . unwrap () ; } assert_eq ! (buf , expected) ; } } # [test] fn test_vec () { test_json (vec ! ["a" , "b"]) ; } # [test] fn test_struct () { # [derive (Serialize)] struct S { f : usize , } test_json (S { f : 256 }) ; } # [test] fn test_enum () { # [derive (Serialize)] enum E { Unit , Newtype (bool) , Tuple (bool , bool) , Struct { t : bool , f : bool } , } test_json (E :: Unit) ; test_json (E :: Newtype (true)) ; test_json (E :: Tuple (true , false)) ; test_json (E :: Struct { t : true , f : false }) ; } # [test] fn test_error_custom () { struct Kaboom ; impl serde :: Serialize for Kaboom { fn serialize < S > (& self , _ : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: Error as _ ; Err (S :: Error :: custom ("kaboom")) } } let obj : & dyn Serialize = & Kaboom ; let err = serde_json :: to_vec (obj) . unwrap_err () ; assert_eq ! (err . to_string () , "kaboom") ; } # [test] fn assert_serialize () { fn assert < T : serde :: Serialize > () { } assert :: < & dyn Serialize > () ; assert :: < & (dyn Serialize + Send) > () ; assert :: < & (dyn Serialize + Sync) > () ; assert :: < & (dyn Serialize + Send + Sync) > () ; assert :: < & (dyn Serialize + Sync + Send) > () ; assert :: < Vec < & dyn Serialize > > () ; assert :: < Vec < & (dyn Serialize + Send) > > () ; assert :: < Box < dyn Serialize > > () ; assert :: < Box < dyn Serialize + Send > > () ; assert :: < Box < dyn Serialize + Sync > > () ; assert :: < Box < dyn Serialize + Send + Sync > > () ; assert :: < Box < dyn Serialize + Sync + Send > > () ; assert :: < Vec < Box < dyn Serialize > > > () ; assert :: < Vec < Box < dyn Serialize + Send > > > () ; } # [test] fn test_dangle () { let mut json_serializer = serde_json :: Serializer :: new (Vec :: new ()) ; let _erased_serializer = < dyn Serializer > :: erase (& mut json_serializer) ; drop (json_serializer) ; } }
    };
}

tests!()