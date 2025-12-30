// Generated macro for AuxEnum (struct)
macro_rules! Depcrate_wit_nonstandardAuxEnum {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxEnum"}
// Dependencies: {}
# [derive (Debug)] pub struct AuxEnum { # [doc = " The name of this enum"] pub name : String , # [doc = " The copied Rust comments to forward to JS"] pub comments : String , # [doc = " A list of variants with their name, value and comments"] # [doc = " and whether typescript bindings should be generated for each variant"] pub variants : Vec < (String , i64 , String) > , # [doc = " Whether typescript bindings should be generated for this enum."] pub generate_typescript : bool , # [doc = " The namespace to export the enum through, if any"] pub js_namespace : Option < Vec < String > > , }
};
}
