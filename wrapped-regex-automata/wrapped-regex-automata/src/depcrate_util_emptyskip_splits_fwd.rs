// Generated macro for skip_splits_fwd (function)
macro_rules! Depcrate_util_emptyskip_splits_fwd {
() => {
// Module: crate::util::empty
// Provides: {"skip_splits_fwd"}
// Dependencies: {}
# [cold] # [inline (never)] pub (crate) fn skip_splits_fwd < T , F > (input : & Input < '_ > , init_value : T , match_offset : usize , find : F ,) -> Result < Option < T > , MatchError > where F : FnMut (& Input < '_ >) -> Result < Option < (T , usize) > , MatchError > , { skip_splits (true , input , init_value , match_offset , find) }
};
}
