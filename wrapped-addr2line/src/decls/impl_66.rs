macro_rules! deps {
    () => {
        MappedLookup!();
        LookupContinuation!();
        LookupResult!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < L : LookupContinuation > LookupResult < L > { # [doc = " Callers that do not handle split DWARF can call `skip_all_loads`"] # [doc = " to fast-forward to the end result. This result is produced with"] # [doc = " the data that is available and may be less accurate than the"] # [doc = " the results that would be produced if the caller did properly"] # [doc = " support split DWARF."] pub fn skip_all_loads (mut self) -> L :: Output { loop { self = match self { LookupResult :: Output (t) => return t , LookupResult :: Load { continuation , .. } => continuation . resume (None) , } ; } } pub (crate) fn map < T , F : FnOnce (L :: Output) -> T > (self , f : F ,) -> LookupResult < MappedLookup < T , L , F > > { match self { LookupResult :: Output (t) => LookupResult :: Output (f (t)) , LookupResult :: Load { load , continuation } => LookupResult :: Load { load , continuation : MappedLookup { original : continuation , mutator : f , } , } , } } pub (crate) fn unwrap (self) -> L :: Output { match self { LookupResult :: Output (t) => t , LookupResult :: Load { .. } => unreachable ! ("Internal API misuse") , } } }
    };
}

impl_66!()