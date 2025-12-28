macro_rules! deps {
    () => {
        WinconBytes!();
        WinconCapture!();
    };
}

macro_rules! WinconBytesIter {
    () => {
        deps!();
        # [doc = " See [`WinconBytes`]"] # [derive (Debug , PartialEq , Eq)] pub struct WinconBytesIter < 's > { bytes : & 's [u8] , parser : & 's mut anstyle_parse :: Parser , capture : & 's mut WinconCapture , }
    };
}

WinconBytesIter!()