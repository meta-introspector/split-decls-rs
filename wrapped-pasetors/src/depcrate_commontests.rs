// Generated macro for tests (module)
macro_rules! Depcrate_commontests {
() => {
// Module: crate::common
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod tests { use alloc :: string :: String ; use alloc :: vec :: Vec ; use serde_derive :: { Deserialize , Serialize } ; use serde_json :: Value ; # [allow (non_snake_case)] # [derive (Serialize , Deserialize , Debug)] pub (crate) struct TestFile { pub (crate) name : String , pub (crate) tests : Vec < PasetoTest > , } # [allow (non_snake_case)] # [derive (Serialize , Deserialize , Debug)] pub (crate) struct PasetoTest { pub (crate) name : String , # [serde (rename (deserialize = "expect-fail"))] pub (crate) expect_fail : bool , pub (crate) key : Option < String > , pub (crate) nonce : Option < String > , # [serde (rename (deserialize = "public-key"))] pub (crate) public_key : Option < String > , # [serde (rename (deserialize = "secret-key"))] pub (crate) secret_key : Option < String > , # [serde (rename (deserialize = "secret-key-seed"))] pub (crate) secret_key_seed : Option < String > , # [serde (rename (deserialize = "public-key-pem"))] pub (crate) public_key_pem : Option < String > , # [serde (rename (deserialize = "secret-key-pem"))] pub (crate) secret_key_pem : Option < String > , pub (crate) token : String , pub (crate) payload : Option < Value > , pub (crate) footer : String , # [serde (rename (deserialize = "implicit-assertion"))] pub (crate) implicit_assertion : String , } # [allow (non_snake_case)] # [derive (Serialize , Deserialize , Debug)] pub (crate) struct Payload { pub (crate) data : String , pub (crate) exp : String , } }
};
}
