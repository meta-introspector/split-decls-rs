// Generated macro for test (module)
macro_rules! Depcrate_serdetest {
() => {
// Module: crate::serde
// Provides: {"test"}
// Dependencies: {}
# [allow (unused_results)] # [cfg (test)] mod test { use coverage_helper :: test ; use serde_test :: { assert_de_tokens_error , assert_tokens , Token } ; use super :: * ; # [test] fn test_de_error () { assert_de_tokens_error :: < VecList < u32 > > (& [Token :: Map { len : Some (0) }] , "invalid type: map, expected a sequence" ,) ; } # [test] fn test_ser_de_empty () { let list = VecList :: < u32 > :: new () ; assert_tokens (& list , & [Token :: Seq { len : Some (0) } , Token :: SeqEnd]) ; } # [test] fn test_ser_de () { let mut list = VecList :: new () ; list . push_back (0) ; list . push_back (1) ; list . push_back (2) ; list . push_back (3) ; list . push_back (4) ; assert_tokens (& list , & [Token :: Seq { len : Some (5) } , Token :: I32 (0) , Token :: I32 (1) , Token :: I32 (2) , Token :: I32 (3) , Token :: I32 (4) , Token :: SeqEnd ,] ,) ; } }
};
}
