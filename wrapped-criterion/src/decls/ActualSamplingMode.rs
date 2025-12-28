macro_rules! ActualSamplingMode {
    () => {
        # [doc = " Enum to represent the sampling mode without Auto."] # [derive (Debug , Clone , Copy , Serialize , Deserialize)] pub (crate) enum ActualSamplingMode { Linear , Flat , }
    };
}

ActualSamplingMode!()