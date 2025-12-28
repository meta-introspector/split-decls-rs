macro_rules! ItemId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Hash , HashStable_Generic)] pub struct ItemId { pub owner_id : OwnerId , }
    };
}

ItemId!();