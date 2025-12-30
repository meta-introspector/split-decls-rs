// Generated macro for parse (function)
macro_rules! Depcrate_parseparse {
() => {
// Module: crate::parse
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse the contents of `src` and return a list of AST types."] pub fn parse () -> Result < types :: Definitions > { let tokens = load_token_file (TOKEN_SRC) ? ; let mut lookup = Lookup { items : BTreeMap :: new () , tokens , aliases : BTreeMap :: new () , } ; load_file (SYN_CRATE_ROOT , & [] , & mut lookup) ? ; let version = version :: get () ? ; let types = lookup . items . values () . map (| item | introspect_item (item , & lookup)) . collect () ; let tokens = lookup . tokens . into_iter () . map (| (name , ty) | (ty , name)) . collect () ; Ok (types :: Definitions { version , types , tokens , }) }
};
}
