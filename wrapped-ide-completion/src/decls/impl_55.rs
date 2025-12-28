macro_rules! deps {
    () => {
        CompletionRelevance!();
        CompletionRelevanceReturnType!();
        CompletionRelevancePostfixMatch!();
        CompletionRelevanceTypeMatch!();
        Builder!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl CompletionRelevance { # [doc = " Provides a relevance score. Higher values are more relevant."] # [doc = ""] # [doc = " The absolute value of the relevance score is not meaningful, for"] # [doc = " example a value of BASE_SCORE doesn't mean \"not relevant\", rather"] # [doc = " it means \"least relevant\". The score value should only be used"] # [doc = " for relative ordering."] # [doc = ""] # [doc = " See is_relevant if you need to make some judgement about score"] # [doc = " in an absolute sense."] const BASE_SCORE : u32 = u32 :: MAX / 2 ; pub fn score (self) -> u32 { let mut score = Self :: BASE_SCORE ; let CompletionRelevance { exact_name_match , type_match , is_local , is_name_already_imported , requires_import , is_private_editable , postfix_match , trait_ , function , is_skipping_completion , } = self ; if is_name_already_imported { score -= 1 ; } if is_local { score += 1 ; } if ! is_private_editable { score += 1 ; } if let Some (trait_) = trait_ { if ! trait_ . notable_trait { score -= 5 ; } if trait_ . is_op_method { score -= 5 ; } } if is_skipping_completion { score -= 7 ; } if requires_import { score -= 1 ; } if exact_name_match { score += 20 ; } match postfix_match { Some (CompletionRelevancePostfixMatch :: Exact) => score += 100 , Some (CompletionRelevancePostfixMatch :: NonExact) => score -= 5 , None => () , } ; score += match type_match { Some (CompletionRelevanceTypeMatch :: Exact) => 18 , Some (CompletionRelevanceTypeMatch :: CouldUnify) => 5 , None => 0 , } ; if let Some (function) = function { let mut fn_score = match function . return_type { CompletionRelevanceReturnType :: DirectConstructor => 15 , CompletionRelevanceReturnType :: Builder => 10 , CompletionRelevanceReturnType :: Constructor => 5 , CompletionRelevanceReturnType :: Other => 0u32 , } ; if function . has_params { fn_score = fn_score . saturating_sub (1) ; } else if function . has_self_param { fn_score = fn_score . min (1) ; } score += fn_score ; } ; score } # [doc = " Returns true when the score for this threshold is above"] # [doc = " some threshold such that we think it is especially likely"] # [doc = " to be relevant."] pub fn is_relevant (& self) -> bool { self . score () > Self :: BASE_SCORE } }
    };
}

impl_55!();