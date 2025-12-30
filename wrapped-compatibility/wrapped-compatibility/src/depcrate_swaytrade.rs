// Generated macro for Trade (struct)
macro_rules! Depcrate_swayTrade {
() => {
// Module: crate::sway
// Provides: {"Trade"}
// Dependencies: {}
# [derive (bincode_2 :: Encode , bincode_2 :: Decode , Serialize , Deserialize , Debug , PartialEq)] # [bincode (crate = "bincode_2")] # [serde (rename_all = "camelCase")] pub struct Trade { pub id : u64 , pub liquidation : bool , pub price : f64 , pub side : TradeSide , pub size : f64 , pub time : String , }
};
}
