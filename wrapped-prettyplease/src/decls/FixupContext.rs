macro_rules! deps {
    () => {
        Precedence!();
    };
}

macro_rules! FixupContext {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct FixupContext { previous_operator : Precedence , next_operator : Precedence , stmt : bool , leftmost_subexpression_in_stmt : bool , match_arm : bool , leftmost_subexpression_in_match_arm : bool , condition : bool , rightmost_subexpression_in_condition : bool , leftmost_subexpression_in_optional_operand : bool , next_operator_can_begin_expr : bool , next_operator_can_continue_expr : bool , next_operator_can_begin_generics : bool , }
    };
}

FixupContext!();