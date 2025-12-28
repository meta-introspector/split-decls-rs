macro_rules! deps {
    () => {
        UndoLog!();
        UnificationTable!();
    };
}

macro_rules! vars_since_snapshot {
    () => {
        deps!();
        fn vars_since_snapshot < 'tcx , T > (table : & UnificationTable < '_ , 'tcx , T > , snapshot_var_len : usize ,) -> Range < T > where T : UnifyKey , super :: UndoLog < 'tcx > : From < sv :: UndoLog < ut :: Delegate < T > > > , { T :: from_index (snapshot_var_len as u32) .. T :: from_index (table . len () as u32) }
    };
}

vars_since_snapshot!()