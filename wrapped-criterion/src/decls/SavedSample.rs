macro_rules! deps {
    () => {
        ActualSamplingMode!();
    };
}

macro_rules! SavedSample {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub (crate) struct SavedSample { sampling_mode : ActualSamplingMode , iters : Vec < f64 > , times : Vec < f64 > , }
    };
}

SavedSample!()