macro_rules! deps {
    () => {
        ResolutionScope!();
        ResolvedRule!();
    };
}

macro_rules! MatchFinder {
    () => {
        deps!();
        # [doc = " Searches a crate for pattern matches and possibly replaces them with something else."] pub struct MatchFinder < 'db > { # [doc = " Our source of information about the user's code."] sema : Semantics < 'db , ide_db :: RootDatabase > , rules : Vec < ResolvedRule < 'db > > , resolution_scope : resolving :: ResolutionScope < 'db > , restrict_ranges : Vec < ide_db :: FileRange > , }
    };
}

MatchFinder!();