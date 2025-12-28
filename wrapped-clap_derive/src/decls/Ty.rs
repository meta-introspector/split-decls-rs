macro_rules! Ty {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub (crate) enum Ty { Unit , Vec , VecVec , Option , OptionOption , OptionVec , OptionVecVec , Other , }
    };
}

Ty!()