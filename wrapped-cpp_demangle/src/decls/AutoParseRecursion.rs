macro_rules! deps {
    () => {
        ParseContext!();
    };
}

macro_rules! AutoParseRecursion {
    () => {
        deps!();
        # [doc = " An RAII type to automatically check the recursion level against the"] # [doc = " maximum. If the maximum has been crossed, return an error. Otherwise,"] # [doc = " increment the level upon construction, and decrement it upon destruction."] struct AutoParseRecursion < 'a > (& 'a ParseContext) ;
    };
}

AutoParseRecursion!()