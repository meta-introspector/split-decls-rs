macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! Slope {
    () => {
        deps!();
        # [doc = " A straight line that passes through the origin `y = m * x`"] # [derive (Clone , Copy)] pub struct Slope < A > (pub A) where A : Float ;
    };
}

Slope!();