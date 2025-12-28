macro_rules! CompletionCandidate {
    () => {
        # [doc = " A shell-agnostic completion candidate"] # [derive (Default , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct CompletionCandidate { value : OsString , help : Option < StyledStr > , id : Option < String > , tag : Option < StyledStr > , display_order : Option < usize > , hidden : bool , }
    };
}

CompletionCandidate!()