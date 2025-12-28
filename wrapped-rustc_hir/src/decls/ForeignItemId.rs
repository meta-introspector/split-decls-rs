macro_rules! ForeignItemId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct ForeignItemId { pub owner_id : OwnerId , }
    };
}

ForeignItemId!();