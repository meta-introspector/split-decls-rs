macro_rules! deps {
    () => {
        RuleType!();
        Affix!();
    };
}

macro_rules! Op {
    () => {
        deps!();
        # [doc = " An operator that corresponds to a rule."] pub struct Op < R : RuleType > { rule : R , affix : Affix , next : Option < Box < Op < R > > > , }
    };
}

Op!();