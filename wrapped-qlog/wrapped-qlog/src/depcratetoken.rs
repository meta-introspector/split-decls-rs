// Generated macro for Token (struct)
macro_rules! DepcrateToken {
() => {
// Module: crate
// Provides: {"Token"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Clone , Serialize , Deserialize , PartialEq , Eq , Debug)] pub struct Token { # [serde (rename (serialize = "type"))] pub ty : Option < TokenType > , pub details : Option < String > , pub raw : Option < events :: RawInfo > , }
};
}
