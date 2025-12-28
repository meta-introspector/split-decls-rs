macro_rules! DoubleFloat {
    () => {
        # [doc = " A larger floating point number represented by two smaller floats."] # [must_use] # [derive (Copy , Clone , PartialEq , PartialOrd , Debug)] pub struct DoubleFloat < F > (F , F) ;
    };
}

DoubleFloat!();