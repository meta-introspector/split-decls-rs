// Generated macro for ParsedAttributes (struct)
macro_rules! Depcrate_attrParsedAttributes {
() => {
// Module: crate::attr
// Provides: {"ParsedAttributes"}
// Dependencies: {}
# [doc = " Parsed attributes in our logical model."] # [derive (Clone)] pub struct ParsedAttributes { # [doc = " If we've been ordered to skip this item."] # [doc = " This is only valid for enum variants."] pub skip : bool , # [doc = " The potential weight assigned to an enum variant."] # [doc = " This must be `None` for things that are not enum variants."] pub weight : Option < u32 > , # [doc = " The mode for `Parameters` to use. See that type for more."] pub params : ParamsMode , # [doc = " The mode for `Strategy` to use. See that type for more."] pub strategy : StratMode , # [doc = " Filter expressions if any."] pub filter : Vec < syn :: Expr > , # [doc = " True if no_bound was specified."] pub no_bound : bool , }
};
}
