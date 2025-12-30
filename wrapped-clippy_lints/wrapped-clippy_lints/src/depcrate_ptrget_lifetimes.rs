// Generated macro for get_lifetimes (function)
macro_rules! Depcrate_ptrget_lifetimes {
() => {
// Module: crate::ptr
// Provides: {"get_lifetimes"}
// Dependencies: {}
# [doc = " Visit `ty` and collect the all the lifetimes appearing in it, implicit or not."] # [doc = ""] # [doc = " The second field of the vector's elements indicate if the lifetime is attached to a"] # [doc = " shared reference, a mutable reference, or neither."] fn get_lifetimes < 'tcx > (ty : & 'tcx hir :: Ty < 'tcx >) -> Vec < (& 'tcx Lifetime , Option < Mutability > , Span) > { use hir :: intravisit :: VisitorExt as _ ; let mut visitor = LifetimeVisitor { result : Vec :: new () } ; visitor . visit_ty_unambig (ty) ; visitor . result }
};
}
