macro_rules! deps {
    () => {
        Ty!();
        Walkable!();
        GenericParam!();
    };
}

macro_rules! UnsafeBinderTy {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct UnsafeBinderTy { pub generic_params : ThinVec < GenericParam > , pub inner_ty : Box < Ty > , }
    };
}

UnsafeBinderTy!()