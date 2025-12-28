macro_rules! deps {
    () => {
        U32Bytes!();
    };
}

macro_rules! ImageAuxSymbolCrc {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolCrc { pub crc : U32Bytes < LE > , }
    };
}

ImageAuxSymbolCrc!();