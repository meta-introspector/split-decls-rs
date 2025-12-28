macro_rules! SnippetScope {
    () => {
        # [doc = " A snippet scope describing where a snippet may apply to."] # [doc = " These may differ slightly in meaning depending on the snippet trigger."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum SnippetScope { Item , Expr , Type , }
    };
}

SnippetScope!()