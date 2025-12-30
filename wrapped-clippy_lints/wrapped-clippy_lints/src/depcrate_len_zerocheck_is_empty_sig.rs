// Generated macro for check_is_empty_sig (function)
macro_rules! Depcrate_len_zerocheck_is_empty_sig {
() => {
// Module: crate::len_zero
// Provides: {"check_is_empty_sig"}
// Dependencies: {}
# [doc = " Checks if the given signature matches the expectations for `is_empty`"] fn check_is_empty_sig < 'tcx > (cx : & LateContext < 'tcx > , sig : FnSig < 'tcx > , self_kind : ImplicitSelfKind , len_output : LenOutput ,) -> bool { match & * * sig . inputs_and_output { [arg , res] if len_output . matches_is_empty_output (cx , * res) => { matches ! ((arg . kind () , self_kind) , (ty :: Ref (_ , _ , Mutability :: Not) , ImplicitSelfKind :: RefImm) | (ty :: Ref (_ , _ , Mutability :: Mut) , ImplicitSelfKind :: RefMut)) || (! arg . is_ref () && matches ! (self_kind , ImplicitSelfKind :: Imm | ImplicitSelfKind :: Mut)) } , _ => false , } }
};
}
