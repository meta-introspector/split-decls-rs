// Generated macro for clear_and_set_capacity (function)
macro_rules! Depcrateclear_and_set_capacity {
() => {
// Module: crate
// Provides: {"clear_and_set_capacity"}
// Dependencies: {}
fn clear_and_set_capacity (buf : & mut Vec < u8 > , cap : usize) -> Result < () , std :: collections :: TryReserveError > { buf . clear () ; if buf . capacity () < cap { buf . try_reserve (cap) ? ; debug_assert ! (buf . capacity () >= cap , "{} >= {}" , buf . capacity () , cap) ; } Ok (()) }
};
}
