macro_rules! deps {
    () => {
        FallbackExtendedS!();
        IeeeFloat!();
    };
}

macro_rules! FallbackExtended {
    () => {
        deps!();
        type FallbackExtended < F > = ieee :: IeeeFloat < FallbackExtendedS < F > > ;
    };
}

FallbackExtended!();