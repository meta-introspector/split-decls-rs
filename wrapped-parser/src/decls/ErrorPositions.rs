macro_rules! deps {
    () => {
        ErrorPositionsInner!();
    };
}

macro_rules! ErrorPositions {
    () => {
        deps!();
        # [doc = " An iterator over the positions inside an error."] # [doc = ""] # [doc = " Constructed from the `Error::positions` function."] # [derive (Debug , Clone)] pub struct ErrorPositions (ErrorPositionsInner) ;
    };
}

ErrorPositions!()