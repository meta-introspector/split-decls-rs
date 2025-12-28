macro_rules! deps {
    () => {
        MetaItem!();
        Lit!();
        MetaItemLit!();
    };
}

macro_rules! MetaItemInner {
    () => {
        deps!();
        # [doc = " Values inside meta item lists."] # [doc = ""] # [doc = " E.g., each of `Clone`, `Copy` in `#[derive(Clone, Copy)]`."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic)] pub enum MetaItemInner { # [doc = " A full MetaItem, for recursive meta items."] MetaItem (MetaItem) , # [doc = " A literal."] # [doc = ""] # [doc = " E.g., `\"foo\"`, `64`, `true`."] Lit (MetaItemLit) , }
    };
}

MetaItemInner!();