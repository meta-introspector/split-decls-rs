macro_rules! NextAction {
    () => {
        # [doc = " A handle to [store](NextAction::store()) or [erase](NextAction::erase()) the outcome of the initial action."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct NextAction { previous_output : BString , }
    };
}

NextAction!()