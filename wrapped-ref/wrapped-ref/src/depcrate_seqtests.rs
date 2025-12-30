// Generated macro for tests (module)
macro_rules! Depcrate_seqtests {
() => {
// Module: crate::seq
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: test :: { compat_case , Ref , Token } ; # [test] fn seq_compat () { compat_case (& [Ref (& 1)] as & [Ref < & i32 >] , & [Token :: SeqBegin (Some (1)) , Token :: SeqValueBegin , Token :: I32 (1) , Token :: SeqValueEnd , Token :: SeqEnd ,] ,) ; compat_case (& [Ref (& 1)] , & [Token :: TaggedBegin (Some (sval :: tags :: CONSTANT_SIZE) , None , None) , Token :: SeqBegin (Some (1)) , Token :: SeqValueBegin , Token :: I32 (1) , Token :: SeqValueEnd , Token :: SeqEnd , Token :: TaggedEnd (Some (sval :: tags :: CONSTANT_SIZE) , None , None) ,] ,) ; compat_case (& (Ref (& 1) , Ref (& 2) , Ref (& 3)) , & [Token :: TupleBegin (None , None , None , Some (3)) , Token :: TupleValueBegin (None , sval :: Index :: new (0)) , Token :: I32 (1) , Token :: TupleValueEnd (None , sval :: Index :: new (0)) , Token :: TupleValueBegin (None , sval :: Index :: new (1)) , Token :: I32 (2) , Token :: TupleValueEnd (None , sval :: Index :: new (1)) , Token :: TupleValueBegin (None , sval :: Index :: new (2)) , Token :: I32 (3) , Token :: TupleValueEnd (None , sval :: Index :: new (2)) , Token :: TupleEnd (None , None , None) ,] ,) ; # [cfg (feature = "std")] { compat_case (& vec ! [Ref (& 1)] , & [Token :: SeqBegin (Some (1)) , Token :: SeqValueBegin , Token :: I32 (1) , Token :: SeqValueEnd , Token :: SeqEnd ,] ,) ; } } }
};
}
