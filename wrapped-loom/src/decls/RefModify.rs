macro_rules! RefModify {
    () => {
        # [doc = " Actions which modify the Arc's reference count"] # [doc = ""] # [doc = " This is used to ascertain dependence for Action::Inspect"] # [derive (Debug , Copy , Clone , PartialEq)] enum RefModify { # [doc = " Corresponds to Action::RefInc"] RefInc , # [doc = " Corresponds to Action::RefDec"] RefDec , }
    };
}

RefModify!();