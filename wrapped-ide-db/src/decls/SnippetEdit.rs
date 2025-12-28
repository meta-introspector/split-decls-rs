macro_rules! SnippetEdit {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct SnippetEdit (Vec < (u32 , TextRange) >) ;
    };
}

SnippetEdit!();