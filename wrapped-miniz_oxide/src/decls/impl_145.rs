macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl State { # [cfg (not (feature = "rustc-dep-of-std"))] const fn is_failure (self) -> bool { matches ! (self , BlockTypeUnexpected | BadCodeSizeSum | BadDistOrLiteralTableLength | BadTotalSymbols | BadZlibHeader | DistanceOutOfBounds | BadRawLength | BadCodeSizeDistPrevLookup | InvalidLitlen | InvalidDist) } # [inline] fn begin (& mut self , new_state : State) { * self = new_state ; } }
    };
}

impl_145!()