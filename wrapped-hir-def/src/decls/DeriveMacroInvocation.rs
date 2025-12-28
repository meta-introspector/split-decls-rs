macro_rules! DeriveMacroInvocation {
    () => {
        # [derive (Debug , PartialEq , Eq)] struct DeriveMacroInvocation { attr_id : AttrId , # [doc = " The `#[derive]` call"] attr_call_id : MacroCallId , derive_call_ids : SmallVec < Option < MacroCallId > , 4 > , }
    };
}

DeriveMacroInvocation!();