macro_rules! PushError {
    () => {
        # [doc = " An error pushing to the submission queue due to it being full."] # [derive (Debug , Clone , PartialEq , Eq)] # [non_exhaustive] pub struct PushError ;
    };
}

PushError!()