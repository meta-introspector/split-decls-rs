macro_rules! SnippetCap {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct SnippetCap { _private : () , }
    };
}

SnippetCap!();