macro_rules! deps {
    () => {
        Machine!();
        Memory!();
    };
}

macro_rules! MemoryKind {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Copy , Clone)] pub enum MemoryKind < T > { # [doc = " Stack memory. Error if deallocated except during a stack pop."] Stack , # [doc = " Memory allocated by `caller_location` intrinsic. Error if ever deallocated."] CallerLocation , # [doc = " Additional memory kinds a machine wishes to distinguish from the builtin ones."] Machine (T) , }
    };
}

MemoryKind!();