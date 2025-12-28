macro_rules! deps {
    () => {
        FullNameRef!();
        TargetRef!();
        Kind!();
        Target!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Target { # [doc = " Returns the kind of the target the ref is pointing to."] pub fn kind (& self) -> Kind { match self { Target :: Symbolic (_) => Kind :: Symbolic , Target :: Object (_) => Kind :: Object , } } # [doc = " Return true if this is a peeled target with a null hash"] pub fn is_null (& self) -> bool { match self { Target :: Object (oid) => oid . is_null () , Target :: Symbolic (_) => false , } } # [doc = " Interpret this owned Target as shared Target"] pub fn to_ref (& self) -> TargetRef < '_ > { match self { Target :: Object (oid) => TargetRef :: Object (oid) , Target :: Symbolic (name) => TargetRef :: Symbolic (name . as_ref ()) , } } # [doc = " Interpret this target as object id which maybe `None` if it is symbolic."] pub fn try_id (& self) -> Option < & oid > { match self { Target :: Symbolic (_) => None , Target :: Object (oid) => Some (oid) , } } # [doc = " Interpret this target as object id or panic if it is symbolic."] pub fn id (& self) -> & oid { match self { Target :: Symbolic (_) => panic ! ("BUG: tries to obtain object id from symbolic target") , Target :: Object (oid) => oid , } } # [doc = " Return the contained object id or panic"] pub fn into_id (self) -> ObjectId { match self { Target :: Symbolic (_) => panic ! ("BUG: expected peeled reference target but found symbolic one") , Target :: Object (oid) => oid , } } # [doc = " Return the contained object id if the target is peeled or itself if it is not."] pub fn try_into_id (self) -> Result < ObjectId , Self > { match self { Target :: Symbolic (_) => Err (self) , Target :: Object (oid) => Ok (oid) , } } # [doc = " Interpret this target as name of the reference it points to which maybe `None` if it an object id."] pub fn try_name (& self) -> Option < & FullNameRef > { match self { Target :: Symbolic (name) => Some (name . as_ref ()) , Target :: Object (_) => None , } } }
    };
}

impl_69!()