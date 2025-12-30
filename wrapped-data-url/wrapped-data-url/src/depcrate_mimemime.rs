// Generated macro for Mime (struct)
macro_rules! Depcrate_mimeMime {
() => {
// Module: crate::mime
// Provides: {"Mime"}
// Dependencies: {}
# [doc = " <https://mimesniff.spec.whatwg.org/#mime-type-representation>"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Mime { pub type_ : String , pub subtype : String , # [doc = " (name, value)"] pub parameters : Vec < (String , String) > , }
};
}
