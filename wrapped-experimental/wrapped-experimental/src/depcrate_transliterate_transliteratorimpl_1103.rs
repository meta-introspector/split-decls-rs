// Generated macro for impl_1103 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1103 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1103"}
// Dependencies: {}
impl SpecialReplacer < '_ > { # [doc = " Estimates the size of the replacement string produced by this Replacer."] fn estimate_size (& self , data : & MatchData , vt : & VarTable) -> usize { match self { Self :: Compound (replacer) => estimate_replacement_size (replacer , data , vt) , Self :: FunctionCall (call) => { estimate_replacement_size (& call . arg , data , vt) } & Self :: BackReference (num) => data . get_segment (num as usize) . len () , Self :: LeftPlaceholderCursor (_) | Self :: RightPlaceholderCursor (_) | Self :: PureCursor => { 0 } } } # [doc = " Applies the replacement from this replacer to `dest`. Also applies any updates to the cursor."] fn replace (& self , dest : & mut Insertable , data : & MatchData , vt : & VarTable , env : & Env) { match self { Self :: Compound (replacer) => replace_str_with_specials (replacer , dest , data , vt , env) , Self :: PureCursor => dest . set_offset_to_here () , & Self :: LeftPlaceholderCursor (num) => { dest . set_offset_to_chars_off_end (num) ; } & Self :: RightPlaceholderCursor (num) => { debug_assert_eq ! (dest . curr_replacement_len () , 0 , "pre-start cursor not the first replacement") ; dest . set_offset_to_chars_off_start (num) ; } & Self :: BackReference (num) => { dest . push_str (data . get_segment (num as usize)) ; } Self :: FunctionCall (call) => { let mut range_aggregator = dest . start_replaceable_adapter () ; replace_str_with_specials (& call . arg , & mut range_aggregator , data , vt , env) ; call . translit . transliterate (range_aggregator . as_replaceable () . child () , env) ; } } } }
};
}
