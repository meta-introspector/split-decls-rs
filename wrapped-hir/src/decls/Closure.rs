macro_rules! deps {
    () => {
        AnyClosureId!();
    };
}

macro_rules! Closure {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct Closure < 'db > { id : AnyClosureId , subst : GenericArgs < 'db > , }
    };
}

Closure!();