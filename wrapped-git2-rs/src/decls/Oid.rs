macro_rules! Oid {
    () => {
        # [doc = " Unique identity of any object (commit, tree, blob, tag)."] # [derive (Copy , Clone)] # [repr (C)] pub struct Oid { raw : raw :: git_oid , }
    };
}

Oid!()