// Generated macro for solana_account (module)
macro_rules! Depcratesolana_account {
() => {
// Module: crate
// Provides: {"solana_account"}
// Dependencies: {}
pub mod solana_account { use solana_pubkey :: Pubkey ; # [derive (Clone)] pub struct Account { pub lamports : u64 , pub data : Vec < u8 > , pub owner : Pubkey , pub executable : bool , } pub trait ReadableAccount : Sized { fn data (& self) -> & [u8] ; } impl ReadableAccount for Account { fn data (& self) -> & [u8] { & self . data } } pub mod state_traits { use super :: Account ; pub trait StateMut < T > { } impl < T > StateMut < T > for Account { } } }
};
}
