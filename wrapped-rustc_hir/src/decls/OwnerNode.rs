macro_rules! deps {
    () => {
        Crate!();
        Item!();
        ForeignItem!();
        TraitItem!();
        Mod!();
        ImplItem!();
    };
}

macro_rules! OwnerNode {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum OwnerNode < 'hir > { Item (& 'hir Item < 'hir >) , ForeignItem (& 'hir ForeignItem < 'hir >) , TraitItem (& 'hir TraitItem < 'hir >) , ImplItem (& 'hir ImplItem < 'hir >) , Crate (& 'hir Mod < 'hir >) , Synthetic , }
    };
}

OwnerNode!();