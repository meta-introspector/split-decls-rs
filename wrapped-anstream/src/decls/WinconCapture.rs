macro_rules! WinconCapture {
    () => {
        # [derive (Default , Clone , Debug , PartialEq , Eq)] struct WinconCapture { style : anstyle :: Style , printable : String , ready : Option < anstyle :: Style > , }
    };
}

WinconCapture!()