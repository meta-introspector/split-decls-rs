macro_rules! TraitItemId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct TraitItemId { pub owner_id : OwnerId , }
    };
}

TraitItemId!()