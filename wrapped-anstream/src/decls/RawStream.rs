macro_rules! deps {
    () => {
        WinconStream!();
        IsTerminal!();
    };
}

macro_rules! RawStream {
    () => {
        deps!();
        # [doc = " Required functionality for underlying [`std::io::Write`] for adaptation"] # [cfg (all (windows , feature = "wincon"))] pub trait RawStream : std :: io :: Write + IsTerminal + anstyle_wincon :: WinconStream + private :: Sealed { }
    };
}

RawStream!();