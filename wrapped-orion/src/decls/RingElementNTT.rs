macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! RingElementNTT {
    () => {
        deps!();
        # [derive (PartialEq , Debug , Clone , Copy)] # [doc = " Element in T_q."] pub struct RingElementNTT { pub coefficients : [FieldElement ; 256] , }
    };
}

RingElementNTT!();