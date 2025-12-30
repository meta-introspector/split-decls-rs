// Generated macro for BitsNext (enum)
macro_rules! DepcrateBitsNext {
() => {
// Module: crate
// Provides: {"BitsNext"}
// Dependencies: {}
enum BitsNext { BlockHeader , BlockUncompressed , BlockFixed , BlockDynHlit , BlockDynHdist (u8) , BlockDynHclen (u8 , u8) , BlockDynClenCodeLengths (u8 , u8 , u8 , u8 , Box < [u8 ; 19] >) , BlockDynCodeLengths (CodeLengthReader) , BlockDyn (DynHuffman16 , DynHuffman16 , u16) }
};
}
