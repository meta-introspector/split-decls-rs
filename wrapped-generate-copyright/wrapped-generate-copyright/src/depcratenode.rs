// Generated macro for Node (enum)
macro_rules! DepcrateNode {
() => {
// Module: crate
// Provides: {"Node"}
// Dependencies: {}
# [doc = " Describes one node in our metadata tree"] # [derive (serde :: Deserialize , Template , Clone , Debug , PartialEq , Eq)] # [serde (rename_all = "kebab-case" , tag = "type")] # [template (path = "Node.html")] pub (crate) enum Node { Root { children : Vec < Node > } , Directory { name : String , children : Vec < Node > , license : Option < License > } , File { name : String , license : License } , Group { files : Vec < String > , directories : Vec < String > , license : License } , }
};
}
