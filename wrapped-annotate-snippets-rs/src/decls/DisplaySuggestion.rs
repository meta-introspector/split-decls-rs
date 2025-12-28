macro_rules! DisplaySuggestion {
    () => {
        # [derive (Clone , Copy , Debug)] pub (crate) enum DisplaySuggestion { Underline , Diff , None , Add , }
    };
}

DisplaySuggestion!()