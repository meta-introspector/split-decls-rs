// Generated macro for Demo (struct)
macro_rules! DepcrateDemo {
() => {
// Module: crate
// Provides: {"Demo"}
// Dependencies: {}
# [cfg (all (test , feature = "serde"))] # [derive (Serialize , Deserialize , Debug , PartialEq)] struct Demo { num : u32 , name : String , enc : & 'static Encoding , }
};
}
