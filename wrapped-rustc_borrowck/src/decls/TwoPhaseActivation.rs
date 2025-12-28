macro_rules! TwoPhaseActivation {
    () => {
        # [doc = " Location where a two-phase borrow is activated, if a borrow"] # [doc = " is in fact a two-phase borrow."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum TwoPhaseActivation { NotTwoPhase , NotActivated , ActivatedAt (Location) , }
    };
}

TwoPhaseActivation!()