// Generated macro for has_implicit_borrow (function)
macro_rules! Depcrate_autorefshas_implicit_borrow {
() => {
// Module: crate::autorefs
// Provides: {"has_implicit_borrow"}
// Dependencies: {}
# [doc = " Test if some adjustment has some implicit borrow."] # [doc = ""] # [doc = " Returns `Some((mutability, was_an_overloaded_deref))` if the argument adjustment is"] # [doc = " an implicit borrow (or has an implicit borrow via an overloaded deref)."] fn has_implicit_borrow (Adjustment { kind , .. } : & Adjustment < '_ >) -> Option < (Mutability , bool) > { match kind { & Adjust :: Deref (Some (OverloadedDeref { mutbl , .. })) => Some ((mutbl , true)) , & Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) => Some ((mutbl . into () , false)) , Adjust :: NeverToAny | Adjust :: Pointer (..) | Adjust :: ReborrowPin (..) | Adjust :: Deref (None) | Adjust :: Borrow (AutoBorrow :: RawPtr (..)) => None , } }
};
}
