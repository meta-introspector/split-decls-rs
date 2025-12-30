// Generated macro for FTXresponse (enum)
macro_rules! Depcrate_swayFTXresponse {
() => {
// Module: crate::sway
// Provides: {"FTXresponse"}
// Dependencies: {}
# [derive (bincode_2 :: Encode , bincode_2 :: Decode , Serialize , Deserialize , Debug , PartialEq , Eq)] # [bincode (crate = "bincode_2")] pub enum FTXresponse < T > { Result (FTXresponseSuccess < T >) , Error (FTXresponseFailure) , }
};
}
