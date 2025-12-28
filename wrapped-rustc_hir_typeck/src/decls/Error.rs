macro_rules! deps {
    () => {
        Issue!();
        Compatibility!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Similar to `Issue`, but contains some extra information"] # [derive (Debug , PartialEq , Eq)] pub (crate) enum Error < 'tcx > { # [doc = " The provided argument is the invalid type for the expected input"] Invalid (ProvidedIdx , ExpectedIdx , Compatibility < 'tcx >) , # [doc = " There is a missing input"] Missing (ExpectedIdx) , # [doc = " There's a superfluous argument"] Extra (ProvidedIdx) , # [doc = " Two arguments should be swapped"] Swap (ProvidedIdx , ProvidedIdx , ExpectedIdx , ExpectedIdx) , # [doc = " Several arguments should be reordered"] Permutation (Vec < (ExpectedIdx , ProvidedIdx) >) , }
    };
}

Error!();