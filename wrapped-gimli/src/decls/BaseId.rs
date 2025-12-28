macro_rules! BaseId {
    () => {
        # [cfg (debug_assertions)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] struct BaseId (usize) ;
    };
}

BaseId!()