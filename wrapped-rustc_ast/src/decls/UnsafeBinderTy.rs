macro_rules! deps {
    () => {
        GenericParam!();
        Ty!();
        Walkable!();
    };
}

macro_rules! UnsafeBinderTy {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct UnsafeBinderTy { pub generic_params : ThinVec < GenericParam > , pub inner_ty : Box < Ty > , }
    };
}

UnsafeBinderTy!();