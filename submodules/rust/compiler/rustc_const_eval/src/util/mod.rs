mkuse!{use rustc_middle :: mir ;}
mkmod!{alignment, { 
                getname!(alignment);
                getsrc!(alignment);
                getpath!(alignment);
                get_deps!(alignment);
                get_crates!(alignment);
                mkinclude!(alignment);
                 
            }}
mkmod!{caller_location, { 
                getname!(caller_location);
                getsrc!(caller_location);
                getpath!(caller_location);
                get_deps!(caller_location);
                get_crates!(caller_location);
                mkinclude!(caller_location);
                 
            }}
mkmod!{check_validity_requirement, { 
                getname!(check_validity_requirement);
                getsrc!(check_validity_requirement);
                getpath!(check_validity_requirement);
                get_deps!(check_validity_requirement);
                get_crates!(check_validity_requirement);
                mkinclude!(check_validity_requirement);
                 
            }}
mkmod!{compare_types, { 
                getname!(compare_types);
                getsrc!(compare_types);
                getpath!(compare_types);
                get_deps!(compare_types);
                get_crates!(compare_types);
                mkinclude!(compare_types);
                 
            }}
mkmod!{type_name, { 
                getname!(type_name);
                getsrc!(type_name);
                getpath!(type_name);
                get_deps!(type_name);
                get_crates!(type_name);
                mkinclude!(type_name);
                 
            }}
mkuse!{pub use self :: alignment :: { is_disaligned , is_within_packed } ;}
mkuse!{pub use self :: check_validity_requirement :: check_validity_requirement ;}
mkuse!{pub (crate) use self :: check_validity_requirement :: validate_scalar_in_layout ;}
mkuse!{pub use self :: compare_types :: { relate_types , sub_types } ;}
mkuse!{pub use self :: type_name :: type_name ;}

macro_rules! binop_left_homogeneous_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function binop_left_homogeneous in module {}", module_path!());
    };
}

mkfn!{
    binop_left_homogeneous_introspect!();
    # [doc = " Classify whether an operator is \"left-homogeneous\", i.e., the LHS has the"] # [doc = " same type as the result."] # [inline] pub fn binop_left_homogeneous (op : mir :: BinOp) -> bool { use rustc_middle :: mir :: BinOp :: * ; match op { Add | AddUnchecked | Sub | SubUnchecked | Mul | MulUnchecked | Div | Rem | BitXor | BitAnd | BitOr | Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => true , AddWithOverflow | SubWithOverflow | MulWithOverflow | Eq | Ne | Lt | Le | Gt | Ge | Cmp => { false } } }
}

macro_rules! binop_right_homogeneous_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function binop_right_homogeneous in module {}", module_path!());
    };
}

mkfn!{
    binop_right_homogeneous_introspect!();
    # [doc = " Classify whether an operator is \"right-homogeneous\", i.e., the RHS has the"] # [doc = " same type as the LHS."] # [inline] pub fn binop_right_homogeneous (op : mir :: BinOp) -> bool { use rustc_middle :: mir :: BinOp :: * ; match op { Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Eq | Ne | Lt | Le | Gt | Ge | Cmp => true , Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => false , } }
}