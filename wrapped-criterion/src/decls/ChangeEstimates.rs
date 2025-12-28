macro_rules! deps {
    () => {
        Estimate!();
    };
}

macro_rules! ChangeEstimates {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Clone)] pub struct ChangeEstimates { pub mean : Estimate , pub median : Estimate , }
    };
}

ChangeEstimates!()