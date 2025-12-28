macro_rules! StatusAnd {
    () => {
        # [doc = " The result of a computation consisting of the output value and the exceptions, if any."] # [must_use] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] pub struct StatusAnd < T > { pub status : Status , pub value : T , }
    };
}

StatusAnd!()