macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! RingElement {
    () => {
        deps!();
        # [derive (PartialEq , Debug , Clone , Copy)] # [doc = " Element in R_q."] # [doc = ""] # [doc = " Ring elements are the same as polynomials, which are the same as vectors"] # [doc = " of coefficients."] # [doc = ""] # [doc = " Adding and subtracting polynomials works the same way in NTT and NTT^{-1},"] # [doc = " it is however illegal to operate on two polynomials from different domains"] # [doc = " at the same time."] pub struct RingElement { pub coefficients : [FieldElement ; 256] , }
    };
}

RingElement!();