// Generated macro for State (enum)
macro_rules! Depcrate_inflate_coreState {
() => {
// Module: crate::inflate::core
// Provides: {"State"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [non_exhaustive] enum State { Start = 0 , ReadZlibCmf , ReadZlibFlg , ReadBlockHeader , BlockTypeNoCompression , RawHeader , RawMemcpy1 , RawMemcpy2 , ReadTableSizes , ReadHufflenTableCodeSize , ReadLitlenDistTablesCodeSize , ReadExtraBitsCodeSize , DecodeLitlen , WriteSymbol , ReadExtraBitsLitlen , DecodeDistance , ReadExtraBitsDistance , RawReadFirstByte , RawStoreFirstByte , WriteLenBytesToEnd , BlockDone , HuffDecodeOuterLoop1 , HuffDecodeOuterLoop2 , ReadAdler32 , DoneForever , BlockTypeUnexpected , BadCodeSizeSum , BadDistOrLiteralTableLength , BadTotalSymbols , BadZlibHeader , DistanceOutOfBounds , BadRawLength , BadCodeSizeDistPrevLookup , InvalidLitlen , InvalidDist , }
};
}
