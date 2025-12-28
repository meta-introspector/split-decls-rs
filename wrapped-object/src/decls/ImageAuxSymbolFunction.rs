macro_rules! deps {
    () => {
        U32Bytes!();
    };
}

macro_rules! ImageAuxSymbolFunction {
    () => {
        deps!();
        # [doc = " Auxiliary symbol format 1: function definitions."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolFunction { pub tag_index : U32Bytes < LE > , pub total_size : U32Bytes < LE > , pub pointer_to_linenumber : U32Bytes < LE > , pub pointer_to_next_function : U32Bytes < LE > , pub unused : [u8 ; 2] , }
    };
}

ImageAuxSymbolFunction!();