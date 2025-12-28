macro_rules! deps {
    () => {
        AcceptFn!();
        AllowedTargets!();
        Stage!();
    };
}

macro_rules! GroupTypeInnerAccept {
    () => {
        deps!();
        pub (super) struct GroupTypeInnerAccept < S : Stage > { pub (super) template : AttributeTemplate , pub (super) accept_fn : AcceptFn < S > , pub (super) allowed_targets : AllowedTargets , pub (super) attribute_type : AttributeType , }
    };
}

GroupTypeInnerAccept!()