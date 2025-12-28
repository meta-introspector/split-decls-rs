macro_rules! ImportInfo {
    () => {
        # [doc = " Information about an import statement found in the AST"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ImportInfo { pub path : String , pub alias : Option < String > , pub is_external : bool , pub source_crate : Option < String > , pub git_source_url : Option < String > , pub git_branch : Option < String > , }
    };
}

ImportInfo!()