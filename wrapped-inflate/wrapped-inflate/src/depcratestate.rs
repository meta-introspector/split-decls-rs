// Generated macro for State (enum)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
enum State { ZlibMethodAndFlags , ZlibFlags (u8) , Bits (BitsNext , BitState) , LenDist ((BitsNext , BitState) , u16 , u16) , Uncompressed (u16) , CheckCRC }
};
}
