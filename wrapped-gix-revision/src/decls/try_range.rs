macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! try_range {
    () => {
        deps!();
        fn try_range (input : & BStr) -> Option < (& [u8] , spec :: Kind) > { input . strip_prefix (b"...") . map (| rest | (rest , spec :: Kind :: ReachableToMergeBase)) . or_else (| | input . strip_prefix (b"..") . map (| rest | (rest , spec :: Kind :: RangeBetween))) }
    };
}

try_range!()