// Generated macro for match_words (function)
macro_rules! Depcratematch_words {
() => {
// Module: crate
// Provides: {"match_words"}
// Dependencies: {}
# [allow (clippy :: iter_skip_zero)] fn match_words < 'a , 'b , B : BitBlock > (a : & 'a BitVec < B > , b : & 'b BitVec < B > ,) -> (MatchWords < 'a , B > , MatchWords < 'b , B >) { let a_len = a . storage () . len () ; let b_len = b . storage () . len () ; if a_len < b_len { (a . blocks () . enumerate () . chain (iter :: repeat (B :: zero ()) . enumerate () . take (b_len) . skip (a_len)) , b . blocks () . enumerate () . chain (iter :: repeat (B :: zero ()) . enumerate () . take (0) . skip (0)) ,) } else { (a . blocks () . enumerate () . chain (iter :: repeat (B :: zero ()) . enumerate () . take (0) . skip (0)) , b . blocks () . enumerate () . chain (iter :: repeat (B :: zero ()) . enumerate () . take (a_len) . skip (b_len)) ,) } }
};
}
