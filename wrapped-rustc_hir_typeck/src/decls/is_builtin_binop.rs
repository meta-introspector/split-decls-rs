macro_rules! deps {
    () => {
        BinOpCategory!();
    };
}

macro_rules! is_builtin_binop {
    () => {
        deps!();
        # [doc = " Returns `true` if this is a built-in arithmetic operation (e.g., u32"] # [doc = " + u32, i16x4 == i16x4) and false if these types would have to be"] # [doc = " overloaded to be legal. There are two reasons that we distinguish"] # [doc = " builtin operations from overloaded ones (vs trying to drive"] # [doc = " everything uniformly through the trait system and intrinsics or"] # [doc = " something like that):"] # [doc = ""] # [doc = " 1. Builtin operations can trivially be evaluated in constants."] # [doc = " 2. For comparison operators applied to SIMD types the result is"] # [doc = "    not of type `bool`. For example, `i16x4 == i16x4` yields a"] # [doc = "    type like `i16x4`. This means that the overloaded trait"] # [doc = "    `PartialEq` is not applicable."] # [doc = ""] # [doc = " Reason #2 is the killer. I tried for a while to always use"] # [doc = " overloaded logic and just check the types in constants/codegen after"] # [doc = " the fact, and it worked fine, except for SIMD types. -nmatsakis"] fn is_builtin_binop < 'tcx > (lhs : Ty < 'tcx > , rhs : Ty < 'tcx > , category : BinOpCategory) -> bool { let (lhs , rhs) = (deref_ty_if_possible (lhs) , deref_ty_if_possible (rhs)) ; match category . into () { BinOpCategory :: Shortcircuit => true , BinOpCategory :: Shift => { lhs . references_error () || rhs . references_error () || lhs . is_integral () && rhs . is_integral () } BinOpCategory :: Math => { lhs . references_error () || rhs . references_error () || lhs . is_integral () && rhs . is_integral () || lhs . is_floating_point () && rhs . is_floating_point () } BinOpCategory :: Bitwise => { lhs . references_error () || rhs . references_error () || lhs . is_integral () && rhs . is_integral () || lhs . is_floating_point () && rhs . is_floating_point () || lhs . is_bool () && rhs . is_bool () } BinOpCategory :: Comparison => { lhs . references_error () || rhs . references_error () || lhs . is_scalar () && rhs . is_scalar () } } }
    };
}

is_builtin_binop!();