macro_rules! Cfg {
    () => {
        # [derive (Default)] pub struct Cfg { features : BTreeSet < & 'static str > , }
    };
}

Cfg!();