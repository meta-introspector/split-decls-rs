macro_rules! deps {
    () => {
        Pairs!();
        Assoc!();
    };
}

macro_rules! PrecClimber {
    () => {
        deps!();
        # [doc = " List of operators and precedences, which can perform [precedence climbing][1] on infix"] # [doc = " expressions contained in a [`Pairs`]. The token pairs contained in the `Pairs` should start"] # [doc = " with a *primary* pair and then alternate between an *operator* and a *primary*."] # [doc = ""] # [doc = " [1]: https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method"] # [doc = " [`Pairs`]: ../iterators/struct.Pairs.html"] # [derive (Debug)] pub struct PrecClimber < R : Clone + 'static > { ops : Cow < 'static , [(R , u32 , Assoc)] > , }
    };
}

PrecClimber!()