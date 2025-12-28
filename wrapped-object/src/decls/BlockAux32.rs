macro_rules! deps {
    () => {
        Symbols!();
        U16!();
    };
}

macro_rules! BlockAux32 {
    () => {
        deps!();
        # [doc = " Block auxiliary entry for the C_BLOCK and C_FCN Symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BlockAux32 { # [doc = " Reserved."] pub pad : [u8 ; 2] , # [doc = " High-order 2 bytes of the source line number."] pub x_lnnohi : U16 < BE > , # [doc = " Low-order 2 bytes of the source line number."] pub x_lnnolo : U16 < BE > , # [doc = " Reserved."] pub pad2 : [u8 ; 12] , }
    };
}

BlockAux32!()