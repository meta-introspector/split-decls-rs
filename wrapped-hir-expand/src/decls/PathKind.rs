macro_rules! PathKind {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum PathKind { Plain , # [doc = " `self::` is `Super(0)`"] Super (u8) , Crate , # [doc = " Absolute path (::foo)"] Abs , # [doc = " `$crate` from macro expansion"] DollarCrate (Crate) , }
    };
}

PathKind!()