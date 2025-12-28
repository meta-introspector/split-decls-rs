macro_rules! deps {
    () => {
        SnippetScope!();
    };
}

macro_rules! Snippet {
    () => {
        deps!();
        # [doc = " A user supplied snippet."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Snippet { pub postfix_triggers : Box < [Box < str >] > , pub prefix_triggers : Box < [Box < str >] > , pub scope : SnippetScope , pub description : Option < Box < str > > , snippet : String , requires : Box < [ModPath] > , }
    };
}

Snippet!()