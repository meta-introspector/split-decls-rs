// Generated macro for macro_7 (macro)
macro_rules! Depcratemacro_7 {
() => {
// Module: crate
// Provides: {"macro_7"}
// Dependencies: {}
define_emoji_macro ! { money , assets : u64 , { pub fn acquire_assets (& mut self , amount : u64) { self . assets += amount ; self . energy += amount / 10 ; println ! ("💰 Acquired {} assets. Total assets: {}. Energy: {}" , amount , self . assets , self . energy) ; } pub fn deploy_assets (& mut self , amount : u64) { if self . assets >= amount { self . assets -= amount ; self . energy = self . energy . saturating_sub (amount / 20) ; println ! ("💸 Deployed {} assets. Remaining assets: {}. Energy: {}" , amount , self . assets , self . energy) ; } else { println ! ("⚠️ Not enough assets to deploy {}!" , amount) ; } } } }
};
}
