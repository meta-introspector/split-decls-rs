// Generated macro for test (module)
macro_rules! Depcrate_serdetest {
() => {
// Module: crate::serde
// Provides: {"test"}
// Dependencies: {}
# [allow (unused_results)] # [cfg (all (test , feature = "std"))] mod test { use coverage_helper :: test ; use serde_test :: { assert_de_tokens_error , assert_tokens , Token } ; use super :: * ; # [test] fn test_de_error () { assert_de_tokens_error :: < ListOrderedMultimap < char , u32 > > (& [Token :: Map { len : Some (0) }] , "invalid type: map, expected a sequence" ,) ; } # [test] fn test_ser_de_empty () { let map = ListOrderedMultimap :: < char , u32 > :: new () ; assert_tokens (& map , & [Token :: Seq { len : Some (0) } , Token :: SeqEnd]) ; } # [test] fn test_ser_de () { let mut map = ListOrderedMultimap :: new () ; map . append ('b' , 20) ; map . append ('a' , 10) ; map . append ('c' , 30) ; map . append ('b' , 30) ; assert_tokens (& map , & [Token :: Seq { len : Some (4) } , Token :: Tuple { len : 2 } , Token :: Char ('b') , Token :: I32 (20) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: Char ('a') , Token :: I32 (10) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: Char ('c') , Token :: I32 (30) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: Char ('b') , Token :: I32 (30) , Token :: TupleEnd , Token :: SeqEnd ,] ,) ; } }
};
}
