macro_rules! CallReceiverAdjust {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum CallReceiverAdjust { Deref , Ref , RefMut , None , }
    };
}

CallReceiverAdjust!()