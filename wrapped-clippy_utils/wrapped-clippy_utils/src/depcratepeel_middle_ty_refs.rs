// Generated macro for peel_middle_ty_refs (function)
macro_rules! Depcratepeel_middle_ty_refs {
() => {
// Module: crate
// Provides: {"peel_middle_ty_refs"}
// Dependencies: {}
# [doc = " Peels off all references on the type. Returns the underlying type and the number of references"] # [doc = " removed."] pub fn peel_middle_ty_refs (mut ty : Ty < '_ >) -> (Ty < '_ > , usize) { let mut count = 0 ; while let rustc_ty :: Ref (_ , dest_ty , _) = ty . kind () { ty = * dest_ty ; count += 1 ; } (ty , count) }
};
}
