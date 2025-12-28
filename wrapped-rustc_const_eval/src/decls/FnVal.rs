macro_rules! FnVal {
    () => {
        # [doc = " The value of a function pointer."] # [derive (Debug , Copy , Clone)] pub enum FnVal < 'tcx , Other > { Instance (Instance < 'tcx >) , Other (Other) , }
    };
}

FnVal!()