macro_rules! FileReferenceNode {
    () => {
        # [derive (Debug , Clone)] pub enum FileReferenceNode { Name (ast :: Name) , NameRef (ast :: NameRef) , Lifetime (ast :: Lifetime) , FormatStringEntry (ast :: String , TextRange) , }
    };
}

FileReferenceNode!()