macro_rules! deps {
    () => {
        PathValue!();
    };
}

macro_rules! TomlTarget {
    () => {
        deps!();
        # [derive (Default , Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlTarget { pub name : Option < String > , pub crate_type : Option < Vec < String > > , # [serde (rename = "crate_type")] pub crate_type2 : Option < Vec < String > > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<String>"))] pub path : Option < PathValue > , pub filename : Option < String > , pub test : Option < bool > , pub doctest : Option < bool > , pub bench : Option < bool > , pub doc : Option < bool > , pub doc_scrape_examples : Option < bool > , pub proc_macro : Option < bool > , # [serde (rename = "proc_macro")] pub proc_macro2 : Option < bool > , pub harness : Option < bool > , pub required_features : Option < Vec < String > > , pub edition : Option < String > , }
    };
}

TomlTarget!();