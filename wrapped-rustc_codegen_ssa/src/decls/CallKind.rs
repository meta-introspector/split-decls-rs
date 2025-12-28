macro_rules! CallKind {
    () => {
        # [doc = " Indicates to the call terminator codegen whether a call"] # [doc = " is a normal call or an explicit tail call."] # [derive (Debug , PartialEq)] enum CallKind { Normal , Tail , }
    };
}

CallKind!()