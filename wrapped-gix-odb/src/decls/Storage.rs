macro_rules! Storage {
    () => {
        # [doc = " A mapping between an object id and all data corresponding to an object, acting like a `HashMap<ObjectID, (Kind, Data)>`."] # [derive (Default , Debug , Clone , Eq , PartialEq)] pub struct Storage (gix_hashtable :: HashMap < gix_hash :: ObjectId , (gix_object :: Kind , Vec < u8 >) >) ;
    };
}

Storage!()