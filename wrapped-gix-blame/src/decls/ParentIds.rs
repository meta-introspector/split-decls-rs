macro_rules! ParentIds {
    () => {
        type ParentIds = SmallVec < (gix_hash :: ObjectId , i64) , 2 > ;
    };
}

ParentIds!()