// Generated macro for tests (module)
macro_rules! Depcrate_de_valuetests {
() => {
// Module: crate::de::value
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: vec ; use core :: str :: FromStr ; use super :: * ; fn eval (s : & str) -> Value { s . parse () . expect ("Failed to parse") } # [test] fn test_none () { assert_eq ! (eval ("None") , Value :: Option (None)) ; } # [test] fn test_some () { assert_eq ! (eval ("Some(())") , Value :: Option (Some (Box :: new (Value :: Unit)))) ; assert_eq ! (eval ("Some  (  () )") , Value :: Option (Some (Box :: new (Value :: Unit)))) ; } # [test] fn test_tuples_basic () { assert_eq ! (eval ("(3, 4.0, 5.0)") , Value :: Seq (vec ! [Value :: Number (Number :: U8 (3)) , Value :: Number (Number :: F32 (4.0 . into ())) , Value :: Number (Number :: F32 (5.0 . into ())) ,] ,) ,) ; } # [test] fn test_tuples_ident () { assert_eq ! (eval ("(true, 3, 4, 5.0)") , Value :: Seq (vec ! [Value :: Bool (true) , Value :: Number (Number :: U8 (3)) , Value :: Number (Number :: U8 (4)) , Value :: Number (Number :: F32 (5.0 . into ())) ,]) ,) ; } # [test] fn test_tuples_error () { use crate :: de :: { Error , Position , Span , SpannedError } ; assert_eq ! (Value :: from_str ("Foo:") . unwrap_err () , SpannedError { code : Error :: TrailingCharacters , span : Span { start : Position { line : 1 , col : 4 } , end : Position { line : 1 , col : 4 } } } ,) ; } # [test] fn test_floats () { assert_eq ! (eval ("(inf, -inf, NaN)") , Value :: Seq (vec ! [Value :: Number (Number :: new (core :: f32 :: INFINITY)) , Value :: Number (Number :: new (core :: f32 :: NEG_INFINITY)) , Value :: Number (Number :: new (core :: f32 :: NAN)) ,]) ,) ; } # [test] fn test_complex () { assert_eq ! (eval ("Some([
    Room ( width: 20, height: 5, name: \"The Room\" ),

    (
        width: 10.0,
        height: 10.0,
        name: \"Another room\",
        enemy_levels: {
            \"Enemy1\": 3,
            \"Enemy2\": 5,
            \"Enemy3\": 7,
        },
    ),
])") , Value :: Option (Some (Box :: new (Value :: Seq (vec ! [Value :: Map (vec ! [(Value :: String ("width" . to_owned ()) , Value :: Number (Number :: U8 (20)) ,) , (Value :: String ("height" . to_owned ()) , Value :: Number (Number :: U8 (5)) ,) , (Value :: String ("name" . to_owned ()) , Value :: String ("The Room" . to_owned ()) ,) ,] . into_iter () . collect () ,) , Value :: Map (vec ! [(Value :: String ("width" . to_owned ()) , Value :: Number (Number :: F32 (10.0 . into ())) ,) , (Value :: String ("height" . to_owned ()) , Value :: Number (Number :: F32 (10.0 . into ())) ,) , (Value :: String ("name" . to_owned ()) , Value :: String ("Another room" . to_owned ()) ,) , (Value :: String ("enemy_levels" . to_owned ()) , Value :: Map (vec ! [(Value :: String ("Enemy1" . to_owned ()) , Value :: Number (Number :: U8 (3)) ,) , (Value :: String ("Enemy2" . to_owned ()) , Value :: Number (Number :: U8 (5)) ,) , (Value :: String ("Enemy3" . to_owned ()) , Value :: Number (Number :: U8 (7)) ,) ,] . into_iter () . collect () ,) ,) ,] . into_iter () . collect () ,) ,]))))) ; } # [test] fn test_struct () { assert_eq ! (eval ("(a:42)") , Value :: Map ([(Value :: String (String :: from ("a")) , Value :: Number (Number :: U8 (42)))] . into_iter () . collect ()) ,) ; assert_eq ! (eval ("(r#a:42)") , Value :: Map ([(Value :: String (String :: from ("a")) , Value :: Number (Number :: U8 (42)))] . into_iter () . collect ()) ,) ; assert_eq ! ("(r#:42)" . parse ::< Value > () . unwrap_err () , crate :: error :: SpannedError { code : crate :: Error :: ExpectedString , span : crate :: error :: Span { start : crate :: error :: Position { line : 1 , col : 3 } , end : crate :: error :: Position { line : 1 , col : 4 } , } } ,) ; assert_eq ! ("( /*" . parse ::< Value > () . unwrap_err () , crate :: error :: SpannedError { code : crate :: Error :: UnclosedBlockComment , span : crate :: error :: Span { start : crate :: error :: Position { line : 1 , col : 3 } , end : crate :: error :: Position { line : 1 , col : 5 } , } } ,) ; } }
};
}
