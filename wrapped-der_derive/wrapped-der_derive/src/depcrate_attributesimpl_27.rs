// Generated macro for impl_27 (impl)
macro_rules! Depcrate_attributesimpl_27 {
() => {
// Module: crate::attributes
// Provides: {"impl_27"}
// Dependencies: {}
impl AttrNameValue { pub fn parse_attribute (attr : & Attribute) -> syn :: Result < impl IntoIterator < Item = Self > > { attr . parse_args_with (Punctuated :: < Self , Token ! [,] > :: parse_terminated) } # [doc = " Parse a slice of attributes."] pub fn from_attributes (attrs : & [Attribute] , out : & mut Vec < Self >) -> syn :: Result < () > { for attr in attrs { if ! attr . path () . is_ident (ATTR_NAME) { continue ; } match Self :: parse_attribute (attr) { Ok (parsed) => out . extend (parsed) , Err (e) => abort ! (attr , e) , } } Ok (()) } # [doc = " Parse an attribute value if the name matches the specified one."] pub fn parse_value < T > (& self , name : & str) -> syn :: Result < Option < T > > where T : FromStr + Debug , T :: Err : Debug , { Ok (if self . name . is_ident (name) { Some (self . value . value () . parse () . map_err (| _ | syn :: Error :: new_spanned (& self . name , "error parsing attribute")) ? ,) } else { None }) } }
};
}
