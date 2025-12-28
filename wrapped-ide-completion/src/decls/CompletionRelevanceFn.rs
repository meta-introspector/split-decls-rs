macro_rules! deps {
    () => {
        CompletionRelevanceReturnType!();
    };
}

macro_rules! CompletionRelevanceFn {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CompletionRelevanceFn { pub has_params : bool , pub has_self_param : bool , pub return_type : CompletionRelevanceReturnType , }
    };
}

CompletionRelevanceFn!()