macro_rules! deps {
    () => {
        LangItemTarget!();
    };
}

macro_rules! LangItems {
    () => {
        deps!();
        # [derive (Default , Debug , Clone , PartialEq , Eq)] pub struct LangItems { items : FxHashMap < LangItem , LangItemTarget > , }
    };
}

LangItems!()