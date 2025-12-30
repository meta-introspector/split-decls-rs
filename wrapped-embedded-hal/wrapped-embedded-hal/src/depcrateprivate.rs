// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: i2c :: { SevenBitAddress , TenBitAddress } ; pub trait Sealed { } impl Sealed for SevenBitAddress { } impl Sealed for TenBitAddress { } }
};
}
