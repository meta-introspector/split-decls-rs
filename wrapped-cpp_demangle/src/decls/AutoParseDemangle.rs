macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleContext!();
    };
}

macro_rules! AutoParseDemangle {
    () => {
        deps!();
        # [doc = " An RAII type to automatically check the recursion level against the"] # [doc = " maximum. If the maximum has been crossed, return an error. Otherwise,"] # [doc = " increment the level upon construction, and decrement it upon destruction."] struct AutoParseDemangle < 'a , 'b , W : 'a + DemangleWrite > (& 'b mut DemangleContext < 'a , W >) ;
    };
}

AutoParseDemangle!();