macro_rules! deps {
    () => {
        ItemId!();
        ModSpans!();
    };
}

macro_rules! Mod {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Mod < 'hir > { pub spans : ModSpans , pub item_ids : & 'hir [ItemId] , }
    };
}

Mod!();