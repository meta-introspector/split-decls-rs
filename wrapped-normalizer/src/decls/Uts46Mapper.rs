macro_rules! deps {
    () => {
        ComposingNormalizer!();
    };
}

macro_rules! Uts46Mapper {
    () => {
        deps!();
        # [doc = " A mapper that knows how to performs the subsets of UTS 46 processing"] # [doc = " documented on the methods."] # [derive (Debug)] pub struct Uts46Mapper { normalizer : ComposingNormalizer , }
    };
}

Uts46Mapper!()