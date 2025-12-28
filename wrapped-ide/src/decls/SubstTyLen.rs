macro_rules! SubstTyLen {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum SubstTyLen { Unlimited , LimitTo (usize) , Hide , }
    };
}

SubstTyLen!();