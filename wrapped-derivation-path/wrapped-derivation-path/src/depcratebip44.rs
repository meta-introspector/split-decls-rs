// Generated macro for Bip44 (trait)
macro_rules! DepcrateBip44 {
() => {
// Module: crate
// Provides: {"Bip44"}
// Dependencies: {}
trait Bip44 { const PURPOSE : u32 = 44 ; const COIN : u32 ; fn base_indexes (& self) -> Vec < ChildIndex > { vec ! [ChildIndex :: Hardened (Self :: PURPOSE) , ChildIndex :: Hardened (Self :: COIN) ,] } }
};
}
