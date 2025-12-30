// Generated macro for peel_and_count_ty_refs (function)
macro_rules! Depcrate_typeel_and_count_ty_refs {
() => {
// Module: crate::ty
// Provides: {"peel_and_count_ty_refs"}
// Dependencies: {}
# [doc = " Peels off all references on the type. Returns the underlying type, the number of references"] # [doc = " removed, and, if there were any such references, whether the pointer is ultimately mutable or"] # [doc = " not."] pub fn peel_and_count_ty_refs (mut ty : Ty < '_ >) -> (Ty < '_ > , usize , Option < Mutability >) { let mut count = 0 ; let mut mutbl = None ; while let ty :: Ref (_ , dest_ty , m) = ty . kind () { ty = * dest_ty ; count += 1 ; mutbl . replace (mutbl . map_or (* m , | mutbl : Mutability | mutbl . min (* m))) ; } (ty , count , mutbl) }
};
}
