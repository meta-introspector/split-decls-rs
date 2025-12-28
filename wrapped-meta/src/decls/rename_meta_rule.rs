macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! rename_meta_rule {
    () => {
        deps!();
        # [doc = " A helper function to rename verbose rules"] # [doc = " for the sake of better error messages"] # [inline] pub fn rename_meta_rule (rule : & Rule) -> String { match * rule { Rule :: grammar_rule => "rule" . to_owned () , Rule :: _push => "PUSH" . to_owned () , Rule :: _push_literal => "PUSH_LITERAL" . to_owned () , Rule :: assignment_operator => "`=`" . to_owned () , Rule :: silent_modifier => "`_`" . to_owned () , Rule :: atomic_modifier => "`@`" . to_owned () , Rule :: compound_atomic_modifier => "`$`" . to_owned () , Rule :: non_atomic_modifier => "`!`" . to_owned () , Rule :: opening_brace => "`{`" . to_owned () , Rule :: closing_brace => "`}`" . to_owned () , Rule :: opening_brack => "`[`" . to_owned () , Rule :: closing_brack => "`]`" . to_owned () , Rule :: opening_paren => "`(`" . to_owned () , Rule :: positive_predicate_operator => "`&`" . to_owned () , Rule :: negative_predicate_operator => "`!`" . to_owned () , Rule :: sequence_operator => "`&`" . to_owned () , Rule :: choice_operator => "`|`" . to_owned () , Rule :: optional_operator => "`?`" . to_owned () , Rule :: repeat_operator => "`*`" . to_owned () , Rule :: repeat_once_operator => "`+`" . to_owned () , Rule :: comma => "`,`" . to_owned () , Rule :: closing_paren => "`)`" . to_owned () , Rule :: quote => "`\"`" . to_owned () , Rule :: insensitive_string => "`^`" . to_owned () , Rule :: range_operator => "`..`" . to_owned () , Rule :: single_quote => "`'`" . to_owned () , Rule :: grammar_doc => "//!" . to_owned () , Rule :: line_doc => "///" . to_owned () , other_rule => format ! ("{:?}" , other_rule) , } }
    };
}

rename_meta_rule!()