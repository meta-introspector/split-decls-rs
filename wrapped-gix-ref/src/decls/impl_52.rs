macro_rules! deps {
    () => {
        PreviousValue!();
        Change!();
        TargetRef!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Change { # [doc = " Return references to values that are the new value after the change is applied, if this is an update."] pub fn new_value (& self) -> Option < crate :: TargetRef < '_ > > { match self { Change :: Update { new , .. } => new . to_ref () . into () , Change :: Delete { .. } => None , } } # [doc = " Return references to values that are in common between all variants and denote the previous observed value."] pub fn previous_value (& self) -> Option < crate :: TargetRef < '_ > > { match self { Change :: Update { expected : PreviousValue :: MustExistAndMatch (previous) | PreviousValue :: ExistingMustMatch (previous) , .. } | Change :: Delete { expected : PreviousValue :: MustExistAndMatch (previous) | PreviousValue :: ExistingMustMatch (previous) , .. } => previous , _ => return None , } . to_ref () . into () } }
    };
}

impl_52!()