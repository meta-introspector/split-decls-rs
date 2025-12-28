macro_rules! NilError {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct NilError ;
    };
}

NilError!()