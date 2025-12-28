macro_rules! deps {
    () => {
        WinconCapture!();
    };
}

macro_rules! WinconBytes {
    () => {
        deps!();
        # [doc = " Incrementally convert to wincon calls for non-contiguous data"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct WinconBytes { parser : anstyle_parse :: Parser , capture : WinconCapture , }
    };
}

WinconBytes!()