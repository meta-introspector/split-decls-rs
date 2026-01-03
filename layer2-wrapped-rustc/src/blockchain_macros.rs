// blockchain_macros.rs - Blockchain-to-Macro-to-Type System
// Generated from airdrop.rs functions by step3_bootstrap

pub use crate::airdrop::*;

#[macro_export]
macro_rules! meme {
    ($name:expr) => {
        crate::airdrop::create_meme_token($name)
    };
}

#[macro_export]
macro_rules! ca {
    ($address:expr) => {
        crate::airdrop::load_contract_address($address)
    };
}

#[macro_export]
macro_rules! holders {
    ($contract:expr) => {
        crate::airdrop::find_token_holders($contract)
    };
}

#[macro_export]
macro_rules! airdrop {
    ($token:expr, $recipients:expr) => {
        crate::airdrop::execute_airdrop($token, $recipients)
    };
}

#[macro_export]
macro_rules! defi {
    (create meme $name:expr, and airdrop to $ca:expr) => {
        airdrop!(meme!($name), holders!(ca!($ca)))
    };
    (pump $token:expr, on $platform:expr) => {
        crate::airdrop::pump_token(&$token, $platform)
    };
}
