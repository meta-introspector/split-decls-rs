// Generated macro for impl_31 (impl)
macro_rules! Depcrate_astimpl_31 {
() => {
// Module: crate::ast
// Provides: {"impl_31"}
// Dependencies: {}
impl Graph < (String , String) > { # [allow (clippy :: result_large_err)] # [doc = " Parses a graph from a file, which path is given. Notice that when doing so, attributes are"] # [doc = " of type `(String, String)`, not the default `(&'a str, &'a str)`, since we are reading and taking"] # [doc = " ownership of the content of the file."] pub fn from_file < 'a , P > (p : P) -> Result < Self , GraphFromFileError < 'a > > where P : AsRef < Path > , { let s = std :: fs :: read_to_string (p) ? ; let mut pairs = DotParser :: parse (Rule :: dotgraph , & s) ? ; let pair = pairs . next () . expect ("The toplevel `Pairs` is empty.") ; match Graph :: try_from (pair) { Ok (g) => Ok (g) , Err (e) => panic ! ("{}" , e) , } } }
};
}
