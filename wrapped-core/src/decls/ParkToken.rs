macro_rules! ParkToken {
    () => {
        # [doc = " A value associated with a parked thread which can be used by `unpark_filter`."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct ParkToken (pub usize) ;
    };
}

ParkToken!();