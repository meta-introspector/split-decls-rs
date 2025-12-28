macro_rules! deps {
    () => {
        ComposingNormalizerBorrowed!();
    };
}

macro_rules! Uts46MapperBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed version of a mapper that knows how to performs the"] # [doc = " subsets of UTS 46 processing documented on the methods."] # [derive (Debug)] pub struct Uts46MapperBorrowed < 'a > { normalizer : ComposingNormalizerBorrowed < 'a > , }
    };
}

Uts46MapperBorrowed!()