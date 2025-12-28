macro_rules! IsSuggestion {
    () => {
        # [doc = " Boolean flag used to indicate if this search is for a suggestion"] # [doc = " or not. If true, we can allow ambiguity and so forth."] # [derive (Clone , Copy , Debug)] pub (crate) struct IsSuggestion (pub bool) ;
    };
}

IsSuggestion!();