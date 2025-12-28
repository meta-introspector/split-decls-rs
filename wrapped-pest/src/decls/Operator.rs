macro_rules! deps {
    () => {
        Assoc!();
        RuleType!();
        PrecClimber!();
    };
}

macro_rules! Operator {
    () => {
        deps!();
        # [doc = " Infix operator used in [`PrecClimber`]."] # [doc = ""] # [doc = " [`PrecClimber`]: struct.PrecClimber.html"] # [derive (Debug)] pub struct Operator < R : RuleType > { rule : R , assoc : Assoc , next : Option < Box < Operator < R > > > , }
    };
}

Operator!();