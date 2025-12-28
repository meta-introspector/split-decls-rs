macro_rules! TargetLoadError {
    () => {
        # [derive (Clone , PartialEq , Eq , Hash)] pub struct TargetLoadError (Arc < str >) ;
    };
}

TargetLoadError!()