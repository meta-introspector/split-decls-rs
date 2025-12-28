macro_rules! deps {
    () => {
        MetaInputValue!();
        ArgsType!();
    };
}

macro_rules! KnownArgumentNames {
    () => {
        deps!();
        # [derive (Default)] pub struct KnownArgumentNames < 'a > { current_args : Option < (& 'a IndexMap < String , MetaInputValue > , ArgsType < 'a >) > , }
    };
}

KnownArgumentNames!();