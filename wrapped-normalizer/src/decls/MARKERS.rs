macro_rules! MARKERS {
    () => {
        # [cfg (feature = "datagen")] # [doc = " The latest minimum set of markers required by this component."] pub const MARKERS : & [DataMarkerInfo] = & [NormalizerNfcV1 :: INFO , NormalizerNfdDataV1 :: INFO , NormalizerNfdTablesV1 :: INFO , NormalizerNfkdDataV1 :: INFO , NormalizerNfkdTablesV1 :: INFO , NormalizerNfdSupplementV1 :: INFO , NormalizerUts46DataV1 :: INFO ,] ;
    };
}

MARKERS!()