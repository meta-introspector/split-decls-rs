macro_rules! TypeMismatchSource {
    () => {
        pub (crate) enum TypeMismatchSource < 'tcx > { # [doc = " Expected the binding to have the given type, but it was found to have"] # [doc = " a different type. Find out when that type first became incompatible."] Ty (Ty < 'tcx >) , # [doc = " When we fail during method argument checking, try to find out if a previous"] # [doc = " expression has constrained the method's receiver in a way that makes the"] # [doc = " argument's type incompatible."] Arg { call_expr : & 'tcx hir :: Expr < 'tcx > , incompatible_arg : usize } , }
    };
}

TypeMismatchSource!()