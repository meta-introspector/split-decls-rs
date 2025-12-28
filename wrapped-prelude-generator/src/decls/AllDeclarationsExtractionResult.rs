macro_rules! deps {
    () => {
        SymbolMap!();
        Declaration!();
    };
}

macro_rules! AllDeclarationsExtractionResult {
    () => {
        deps!();
        # [derive (Debug)] pub struct AllDeclarationsExtractionResult { pub declarations : Vec < split_expanded_lib :: Declaration > , pub symbol_map : crate :: symbol_map :: SymbolMap , pub errors : Vec < split_expanded_lib :: ErrorSample > , pub file_metadata : split_expanded_lib :: FileMetadata , pub public_symbols : Vec < split_expanded_lib :: PublicSymbol > , }
    };
}

AllDeclarationsExtractionResult!();