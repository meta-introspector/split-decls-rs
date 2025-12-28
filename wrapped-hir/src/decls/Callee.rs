macro_rules! deps {
    () => {
        Closure!();
    };
}

macro_rules! Callee {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash , Debug)] enum Callee < 'db > { Def (CallableDefId) , Closure (InternedClosureId , GenericArgs < 'db >) , CoroutineClosure (InternedCoroutineId , GenericArgs < 'db >) , FnPtr , FnImpl (FnTrait) , }
    };
}

Callee!();