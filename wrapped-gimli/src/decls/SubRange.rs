macro_rules! SubRange {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] struct SubRange < T > where T : CloneStableDeref < Target = [u8] > + Debug , { bytes : T , ptr : * const u8 , len : usize , }
    };
}

SubRange!();