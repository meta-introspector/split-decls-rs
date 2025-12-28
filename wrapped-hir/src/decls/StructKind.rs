macro_rules! StructKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum StructKind { Record , Tuple , Unit , }
    };
}

StructKind!();