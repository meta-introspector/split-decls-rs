macro_rules! deps {
    () => {
        Abbreviations!();
        Result!();
    };
}

macro_rules! AbbreviationsCache {
    () => {
        deps!();
        # [doc = " A cache of previously parsed `Abbreviations`."] # [derive (Debug , Default)] pub struct AbbreviationsCache { abbreviations : btree_map :: BTreeMap < u64 , Result < Arc < Abbreviations > > > , }
    };
}

AbbreviationsCache!();