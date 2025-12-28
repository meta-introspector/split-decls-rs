macro_rules! FlowStep {
    () => {
        # [doc = " Describes the next flow step."] # [derive (Debug , Clone)] pub enum FlowStep < C , R > { # [doc = " Just yields an item and continues standard flow."] Continue (C) , # [doc = " Immediately returns an underlying item from the function."] Return (R) , }
    };
}

FlowStep!()