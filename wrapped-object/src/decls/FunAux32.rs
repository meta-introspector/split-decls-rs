macro_rules! deps {
    () => {
        Symbol!();
        U16!();
        U32!();
        File!();
    };
}

macro_rules! FunAux32 {
    () => {
        deps!();
        # [doc = " Function auxiliary entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FunAux32 { # [doc = " File offset to exception table entry."] pub x_exptr : U32 < BE > , # [doc = " Size of function in bytes."] pub x_fsize : U32 < BE > , # [doc = " File pointer to line number"] pub x_lnnoptr : U32 < BE > , # [doc = " Symbol table index of next entry beyond this function."] pub x_endndx : U32 < BE > , # [doc = " Pad"] pub pad : U16 < BE > , }
    };
}

FunAux32!()