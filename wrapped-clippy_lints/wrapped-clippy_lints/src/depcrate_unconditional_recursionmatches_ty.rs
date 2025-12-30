// Generated macro for matches_ty (function)
macro_rules! Depcrate_unconditional_recursionmatches_ty {
() => {
// Module: crate::unconditional_recursion
// Provides: {"matches_ty"}
// Dependencies: {}
# [doc = " When we have `x == y` where `x = &T` and `y = &T`, then that resolves to"] # [doc = " `<&T as PartialEq<&T>>::eq`, which is not the same as `<T as PartialEq<T>>::eq`,"] # [doc = " however we still would want to treat it the same, because we know that it's a blanket impl"] # [doc = " that simply delegates to the `PartialEq` impl with one reference removed."] # [doc = ""] # [doc = " Still, we can't just do `lty.peel_refs() == rty.peel_refs()` because when we have `x = &T` and"] # [doc = " `y = &&T`, this is not necessarily the same as `<T as PartialEq<T>>::eq`"] # [doc = ""] # [doc = " So to avoid these FNs and FPs, we keep removing a layer of references from *both* sides"] # [doc = " until both sides match the expected LHS and RHS type (or they don't)."] fn matches_ty < 'tcx > (mut left : Ty < 'tcx > , mut right : Ty < 'tcx > , expected_left : Ty < 'tcx > , expected_right : Ty < 'tcx > ,) -> bool { while let (& ty :: Ref (_ , lty , _) , & ty :: Ref (_ , rty , _)) = (left . kind () , right . kind ()) { if lty == expected_left && rty == expected_right { return true ; } left = lty ; right = rty ; } false }
};
}
