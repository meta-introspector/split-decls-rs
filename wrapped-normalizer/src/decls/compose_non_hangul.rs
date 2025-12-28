macro_rules! deps {
    () => {
        Composition!();
    };
}

macro_rules! compose_non_hangul {
    () => {
        deps!();
        # [doc = " Performs (non-Hangul) canonical composition on a pair of characters"] # [doc = " or returns `None` if these characters don't compose. Composition"] # [doc = " exclusions are taken into account."] fn compose_non_hangul (mut iter : Char16TrieIterator , starter : char , second : char) -> Option < char > { match iter . next (second) { TrieResult :: NoMatch => None , TrieResult :: NoValue => match iter . next (starter) { TrieResult :: NoMatch => None , TrieResult :: FinalValue (i) => { if let Some (c) = char :: from_u32 (i as u32) { Some (c) } else { debug_assert ! (false) ; None } } TrieResult :: NoValue | TrieResult :: Intermediate (_) => { debug_assert ! (false) ; None } } , TrieResult :: FinalValue (_) | TrieResult :: Intermediate (_) => { debug_assert ! (false) ; None } } }
    };
}

compose_non_hangul!()