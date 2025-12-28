macro_rules! deps {
    () => {
        Estimate!();
    };
}

macro_rules! Estimates {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Clone)] pub struct Estimates { pub mean : Estimate , pub median : Estimate , pub median_abs_dev : Estimate , pub slope : Option < Estimate > , pub std_dev : Estimate , }
    };
}

Estimates!();