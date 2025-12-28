macro_rules! SamplingMethod {
    () => {
        # [doc = " Currently not used; defined for forwards compatibility with cargo-criterion."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub enum SamplingMethod { Linear , Flat , }
    };
}

SamplingMethod!()