macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypeTree {
    () => {
        deps!();
        # [derive (Debug)] pub struct TypeTree { pub namespace : & 'static str , pub nested : BTreeMap < & 'static str , Self > , pub types : BTreeSet < Type > , }
    };
}

TypeTree!()