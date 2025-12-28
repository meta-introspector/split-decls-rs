macro_rules! deps {
    () => {
        NoBoundsException!();
    };
}

macro_rules! UnusedParens {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct UnusedParens { with_self_ty_parens : bool , # [doc = " `1 as (i32) < 2` parses to ExprKind::Lt"] # [doc = " `1 as i32 < 2` parses to i32::<2[missing angle bracket]"] parens_in_cast_in_lt : Vec < ast :: NodeId > , # [doc = " Ty nodes in this map are in TypeNoBounds position. Any bounds they"] # [doc = " contain may be ambiguous w/r/t trailing `+` operators."] in_no_bounds_pos : FxHashMap < ast :: NodeId , NoBoundsException > , }
    };
}

UnusedParens!();