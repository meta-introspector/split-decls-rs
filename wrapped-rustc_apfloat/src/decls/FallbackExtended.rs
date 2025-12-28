macro_rules! deps {
    () => {
        IeeeFloat!();
        FallbackExtendedS!();
    };
}

macro_rules! FallbackExtended {
    () => {
        deps!();
        type FallbackExtended < F > = ieee :: IeeeFloat < FallbackExtendedS < F > > ;
    };
}

FallbackExtended!()