macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! AugmentationData {
    () => {
        deps!();
        # [doc = " Parsed augmentation data for a `FrameDescriptEntry`."] # [derive (Clone , Debug , Default , PartialEq , Eq)] struct AugmentationData { lsda : Option < Pointer > , }
    };
}

AugmentationData!();