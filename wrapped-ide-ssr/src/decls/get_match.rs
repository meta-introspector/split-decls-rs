macro_rules! deps {
    () => {
        MatchFailed!();
        Match!();
        Matcher!();
        ResolvedRule!();
    };
}

macro_rules! get_match {
    () => {
        deps!();
        # [doc = " Checks if `code` matches the search pattern found in `search_scope`, returning information about"] # [doc = " the match, if it does. Since we only do matching in this module and searching is done by the"] # [doc = " parent module, we don't populate nested matches."] pub (crate) fn get_match < 'db > (debug_active : bool , rule : & ResolvedRule < 'db > , code : & SyntaxNode , restrict_range : & Option < FileRange > , sema : & Semantics < 'db , ide_db :: RootDatabase > ,) -> Result < Match , MatchFailed > { record_match_fails_reasons_scope (debug_active , | | { Matcher :: try_match (rule , code , restrict_range , sema) }) }
    };
}

get_match!()