macro_rules! deps {
    () => {
        Register!();
        RegisterRule!();
        ReaderOffset!();
    };
}

macro_rules! RegisterRuleIter {
    () => {
        deps!();
        # [doc = " An unordered iterator for register rules."] # [derive (Debug , Clone)] pub struct RegisterRuleIter < 'iter , T > (:: core :: slice :: Iter < 'iter , (Register , RegisterRule < T >) >) where T : ReaderOffset ;
    };
}

RegisterRuleIter!()