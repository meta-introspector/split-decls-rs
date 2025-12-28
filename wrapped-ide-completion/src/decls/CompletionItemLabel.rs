macro_rules! CompletionItemLabel {
    () => {
        # [derive (Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct CompletionItemLabel { # [doc = " The primary label for the completion item."] pub primary : SmolStr , # [doc = " The left detail for the completion item, usually rendered right next to the primary label."] pub detail_left : Option < String > , # [doc = " The right detail for the completion item, usually rendered right aligned at the end of the completion item."] pub detail_right : Option < String > , }
    };
}

CompletionItemLabel!();