macro_rules! deps {
    () => {
        RawStream!();
        StripStream!();
        WinconStream!();
    };
}

macro_rules! StreamInner {
    () => {
        deps!();
        # [derive (Debug)] enum StreamInner < S : RawStream > { PassThrough (S) , Strip (StripStream < S >) , # [cfg (all (windows , feature = "wincon"))] Wincon (WinconStream < S >) , }
    };
}

StreamInner!()