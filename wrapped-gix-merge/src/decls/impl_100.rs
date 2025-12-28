macro_rules! deps {
    () => {
        TreatAsUnresolved!();
        Outcome!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Outcome < '_ > { # [doc = " Return `true` if there is any conflict that would still need to be resolved as they would yield undesirable trees."] # [doc = " This is based on `how` to determine what should be considered unresolved."] pub fn has_unresolved_conflicts (& self , how : TreatAsUnresolved) -> bool { self . conflicts . iter () . any (| c | c . is_unresolved (how)) } # [doc = " Returns `true` if `index` changed as we applied conflicting stages to it, using `how` to determine if a"] # [doc = " conflict should be considered unresolved."] # [doc = " `removal_mode` decides how unconflicted entries should be removed if they are superseded by"] # [doc = " their conflicted counterparts."] # [doc = " It's important that `index` is at the state of [`Self::tree`]."] # [doc = ""] # [doc = " Note that in practice, whenever there is a single [conflict](Conflict), this function will return `true`."] pub fn index_changed_after_applying_conflicts (& self , index : & mut gix_index :: State , how : TreatAsUnresolved , removal_mode : apply_index_entries :: RemovalMode ,) -> bool { apply_index_entries (& self . conflicts , how , index , removal_mode) } }
    };
}

impl_100!()