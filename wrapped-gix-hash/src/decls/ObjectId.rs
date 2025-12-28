macro_rules! ObjectId {
    () => {
        # [doc = " An owned hash identifying objects, most commonly `Sha1`"] # [derive (PartialEq , Eq , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [non_exhaustive] pub enum ObjectId { # [doc = " A SHA 1 hash digest"] Sha1 ([u8 ; SIZE_OF_SHA1_DIGEST]) , }
    };
}

ObjectId!()