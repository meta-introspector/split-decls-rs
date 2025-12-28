macro_rules! ConfusablesParser {
    () => {
        # [derive (Default)] pub (crate) struct ConfusablesParser { confusables : ThinVec < Symbol > , first_span : Option < Span > , }
    };
}

ConfusablesParser!();