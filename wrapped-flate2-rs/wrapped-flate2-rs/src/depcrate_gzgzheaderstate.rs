// Generated macro for GzHeaderState (enum)
macro_rules! Depcrate_gzGzHeaderState {
() => {
// Module: crate::gz
// Provides: {"GzHeaderState"}
// Dependencies: {}
# [derive (Debug , Default)] pub enum GzHeaderState { Start (u8 , [u8 ; 10]) , Xlen (Option < Box < Crc > > , u8 , [u8 ; 2]) , Extra (Option < Box < Crc > > , u16) , Filename (Option < Box < Crc > >) , Comment (Option < Box < Crc > >) , Crc (Option < Box < Crc > > , u8 , [u8 ; 2]) , # [default] Complete , }
};
}
