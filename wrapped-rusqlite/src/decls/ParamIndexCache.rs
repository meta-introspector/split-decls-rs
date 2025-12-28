macro_rules! deps {
    () => {
        SmallCString!();
    };
}

macro_rules! ParamIndexCache {
    () => {
        deps!();
        # [doc = " Maps parameter names to parameter indices."] # [derive (Default , Clone , Debug)] pub (crate) struct ParamIndexCache (RefCell < BTreeMap < SmallCString , usize > >) ;
    };
}

ParamIndexCache!()