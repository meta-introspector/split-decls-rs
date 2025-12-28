macro_rules! NakedParser {
    () => {
        # [derive (Default)] pub (crate) struct NakedParser { span : Option < Span > , }
    };
}

NakedParser!();