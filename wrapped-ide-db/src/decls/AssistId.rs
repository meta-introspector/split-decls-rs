macro_rules! deps {
    () => {
        AssistKind!();
    };
}

macro_rules! AssistId {
    () => {
        deps!();
        # [doc = " Unique identifier of the assist, should not be shown to the user"] # [doc = " directly."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct AssistId (pub & 'static str , pub AssistKind , pub Option < usize >) ;
    };
}

AssistId!();