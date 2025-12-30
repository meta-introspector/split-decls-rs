// Generated macro for Crc (struct)
macro_rules! DepcrateCrc {
() => {
// Module: crate
// Provides: {"Crc"}
// Dependencies: {}
# [doc = " Crc instance with a specific width, algorithm, and implementation."] # [derive (Clone)] pub struct Crc < W : Width , I : Implementation = DefaultImpl > { pub algorithm : & 'static Algorithm < W > , data : I :: Data < W > , }
};
}
