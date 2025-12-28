macro_rules! RealPredicate {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum RealPredicate { RealPredicateFalse , RealOEQ , RealOGT , RealOGE , RealOLT , RealOLE , RealONE , RealORD , RealUNO , RealUEQ , RealUGT , RealUGE , RealULT , RealULE , RealUNE , RealPredicateTrue , }
    };
}

RealPredicate!()