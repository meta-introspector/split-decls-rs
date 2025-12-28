macro_rules! ignore_unmerged_and_intent_to_add {
    () => {
        fn ignore_unmerged_and_intent_to_add < 'rhs , 'lhs : 'rhs > ((_idx , _path , entry) : (usize , & 'rhs BStr , & 'rhs gix_index :: Entry) ,) -> bool { let stage = entry . stage () ; entry . flags . contains (gix_index :: entry :: Flags :: INTENT_TO_ADD) || stage != gix_index :: entry :: Stage :: Unconflicted }
    };
}

ignore_unmerged_and_intent_to_add!()