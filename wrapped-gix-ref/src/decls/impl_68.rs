macro_rules! deps {
    () => {
        TargetRef!();
        Kind!();
        FullNameRef!();
        Target!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl TargetRef < '_ > { # [doc = " Returns the kind of the target the ref is pointing to."] pub fn kind (& self) -> Kind { match self { TargetRef :: Symbolic (_) => Kind :: Symbolic , TargetRef :: Object (_) => Kind :: Object , } } # [doc = " Interpret this target as object id which maybe `None` if it is symbolic."] pub fn try_id (& self) -> Option < & oid > { match self { TargetRef :: Symbolic (_) => None , TargetRef :: Object (oid) => Some (oid) , } } # [doc = " Interpret this target as object id or **panic** if it is symbolic."] pub fn id (& self) -> & oid { match self { TargetRef :: Symbolic (_) => panic ! ("BUG: tries to obtain object id from symbolic target") , TargetRef :: Object (oid) => oid , } } # [doc = " Interpret this target as name of the reference it points to which maybe `None` if it an object id."] pub fn try_name (& self) -> Option < & FullNameRef > { match self { TargetRef :: Symbolic (name) => Some (name) , TargetRef :: Object (_) => None , } } # [doc = " Convert this instance into an owned version, without consuming it."] pub fn into_owned (self) -> Target { self . into () } }
    };
}

impl_68!();