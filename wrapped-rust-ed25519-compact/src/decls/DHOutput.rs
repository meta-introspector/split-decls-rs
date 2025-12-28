macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! DHOutput {
    () => {
        deps!();
        # [doc = " Non-uniform output of a scalar multiplication."] # [doc = " This represents a point on the curve, and should not be used directly as a"] # [doc = " cipher key."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct DHOutput ([u8 ; DHOutput :: BYTES]) ;
    };
}

DHOutput!()