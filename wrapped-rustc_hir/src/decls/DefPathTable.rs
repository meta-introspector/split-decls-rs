macro_rules! deps {
    () => {
        DefPathHashMap!();
        DefKey!();
    };
}

macro_rules! DefPathTable {
    () => {
        deps!();
        # [doc = " The `DefPathTable` maps `DefIndex`es to `DefKey`s and vice versa."] # [doc = " Internally the `DefPathTable` holds a tree of `DefKey`s, where each `DefKey`"] # [doc = " stores the `DefIndex` of its parent."] # [doc = " There is one `DefPathTable` for each crate."] # [derive (Debug)] pub struct DefPathTable { stable_crate_id : StableCrateId , index_to_key : IndexVec < DefIndex , DefKey > , def_path_hashes : IndexVec < DefIndex , Hash64 > , def_path_hash_to_index : DefPathHashMap , }
    };
}

DefPathTable!()