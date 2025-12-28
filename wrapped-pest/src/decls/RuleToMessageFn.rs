macro_rules! RuleToMessageFn {
    () => {
        # [doc = " Function mapping rule to its helper message defined by user."] pub type RuleToMessageFn < R > = Box < dyn Fn (& R) -> Option < String > > ;
    };
}

RuleToMessageFn!()