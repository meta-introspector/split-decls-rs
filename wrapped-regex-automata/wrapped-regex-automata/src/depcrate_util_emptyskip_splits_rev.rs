// Generated macro for skip_splits_rev (function)
macro_rules! Depcrate_util_emptyskip_splits_rev {
() => {
// Module: crate::util::empty
// Provides: {"skip_splits_rev"}
// Dependencies: {}
# [cold] # [inline (never)] pub (crate) fn skip_splits_rev < T , F > (input : & Input < '_ > , init_value : T , match_offset : usize , find : F ,) -> Result < Option < T > , MatchError > where F : FnMut (& Input < '_ >) -> Result < Option < (T , usize) > , MatchError > , { skip_splits (false , input , init_value , match_offset , find) }
};
}
