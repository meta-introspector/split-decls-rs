macro_rules! CaptureUsages {
    () => {
        # [derive (Debug , Clone)] pub struct CaptureUsages { parent : DefWithBodyId , spans : SmallVec < [mir :: MirSpan ; 3] > , }
    };
}

CaptureUsages!()