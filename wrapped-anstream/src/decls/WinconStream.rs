macro_rules! deps {
    () => {
        WinconBytes!();
    };
}

macro_rules! WinconStream {
    () => {
        deps!();
        # [doc = " Only pass printable data to the inner `Write`"] # [cfg (feature = "wincon")] # [derive (Debug)] pub struct WinconStream < S > where S : anstyle_wincon :: WinconStream , { raw : S , state : Box < WinconBytes > , }
    };
}

WinconStream!()