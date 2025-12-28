macro_rules! deps {
    () => {
        AssistId!();
        AssistKind!();
    };
}

macro_rules! SingleResolve {
    () => {
        deps!();
        # [doc = " Hold the [`AssistId`] data of a certain assist to resolve."] # [doc = " The original id object cannot be used due to a `'static` lifetime"] # [doc = " and the requirement to construct this struct dynamically during the resolve handling."] # [derive (Debug)] pub struct SingleResolve { # [doc = " The id of the assist."] pub assist_id : String , pub assist_kind : AssistKind , # [doc = " Subtype of the assist. When many assists have the same id, it differentiates among them."] pub assist_subtype : Option < usize > , }
    };
}

SingleResolve!()