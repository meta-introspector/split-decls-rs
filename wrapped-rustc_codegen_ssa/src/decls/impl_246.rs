macro_rules! deps {
    () => {
        Message!();
        ExtraBackendMethods!();
        Coordinator!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < B : ExtraBackendMethods > Drop for Coordinator < B > { fn drop (& mut self) { if let Some (future) = self . future . take () { drop (self . sender . send (Message :: CodegenAborted :: < B >)) ; drop (future . join ()) ; } } }
    };
}

impl_246!();