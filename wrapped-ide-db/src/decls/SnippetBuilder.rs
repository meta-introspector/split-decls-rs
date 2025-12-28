macro_rules! deps {
    () => {
        PlaceSnippet!();
    };
}

macro_rules! SnippetBuilder {
    () => {
        deps!();
        # [derive (Default)] pub struct SnippetBuilder { # [doc = " Where to place snippets at"] places : Vec < PlaceSnippet > , }
    };
}

SnippetBuilder!()