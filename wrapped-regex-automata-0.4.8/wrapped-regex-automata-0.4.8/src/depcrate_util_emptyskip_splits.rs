// Generated macro for skip_splits (function)
macro_rules! Depcrate_util_emptyskip_splits {
() => {
// Module: crate::util::empty
// Provides: {"skip_splits"}
// Dependencies: {}
fn skip_splits < T , F > (forward : bool , input : & Input < '_ > , init_value : T , mut match_offset : usize , mut find : F ,) -> Result < Option < T > , MatchError > where F : FnMut (& Input < '_ >) -> Result < Option < (T , usize) > , MatchError > , { if input . get_anchored () . is_anchored () { return Ok (if input . is_char_boundary (match_offset) { Some (init_value) } else { None }) ; } let mut value = init_value ; let mut input = input . clone () ; while ! input . is_char_boundary (match_offset) { if forward { input . set_start (input . start () . checked_add (1) . unwrap ()) ; } else { input . set_end (match input . end () . checked_sub (1) { None => return Ok (None) , Some (end) => end , }) ; } match find (& input) ? { None => return Ok (None) , Some ((new_value , new_match_end)) => { value = new_value ; match_offset = new_match_end ; } } } Ok (Some (value)) }
};
}
