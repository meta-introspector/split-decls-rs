// Generated macro for peel_mid_ty_refs_is_mutable (function)
macro_rules! Depcrate_typeel_mid_ty_refs_is_mutable {
() => {
// Module: crate::ty
// Provides: {"peel_mid_ty_refs_is_mutable"}
// Dependencies: {}
# [doc = " Peels off all references on the type. Returns the underlying type, the number of references"] # [doc = " removed, and whether the pointer is ultimately mutable or not."] pub fn peel_mid_ty_refs_is_mutable (ty : Ty < '_ >) -> (Ty < '_ > , usize , Mutability) { fn f (ty : Ty < '_ > , count : usize , mutability : Mutability) -> (Ty < '_ > , usize , Mutability) { match ty . kind () { ty :: Ref (_ , ty , Mutability :: Mut) => f (* ty , count + 1 , mutability) , ty :: Ref (_ , ty , Mutability :: Not) => f (* ty , count + 1 , Mutability :: Not) , _ => (ty , count , mutability) , } } f (ty , 0 , Mutability :: Mut) }
};
}
