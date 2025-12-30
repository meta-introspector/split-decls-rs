// Generated macro for sim (module)
macro_rules! Depcratesim {
() => {
// Module: crate
// Provides: {"sim"}
// Dependencies: {}
mod sim { use core :: fmt ; use arduino_hal :: avr_device ; pub fn exit (_code : u32) -> ! { avr_device :: interrupt :: disable () ; avr_device :: asm :: sleep () ; # [allow (clippy :: empty_loop)] loop { } } pub struct Usart < USART , RX , TX > (pub arduino_hal :: usart :: Usart < USART , RX , TX >) where USART : arduino_hal :: usart :: UsartOps < arduino_hal :: hal :: Atmega , RX , TX > ; impl < USART , RX , TX > fmt :: Write for Usart < USART , RX , TX > where USART : arduino_hal :: usart :: UsartOps < arduino_hal :: hal :: Atmega , RX , TX > , { fn write_str (& mut self , s : & str) -> fmt :: Result { for b in s . bytes () { self . 0 . write_byte (b) ; } Ok (()) } } }
};
}
