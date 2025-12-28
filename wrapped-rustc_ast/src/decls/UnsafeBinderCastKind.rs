macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! UnsafeBinderCastKind {
    () => {
        deps!();
        # [doc = " Whether we're unwrapping or wrapping an unsafe binder"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub enum UnsafeBinderCastKind { Wrap , Unwrap , }
    };
}

UnsafeBinderCastKind!();