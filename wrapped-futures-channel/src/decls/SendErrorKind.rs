macro_rules! SendErrorKind {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] enum SendErrorKind { Full , Disconnected , }
    };
}

SendErrorKind!()