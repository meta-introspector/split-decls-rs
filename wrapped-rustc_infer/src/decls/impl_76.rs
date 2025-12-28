macro_rules! deps {
    () => {
        OpaqueTypeStorage!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'tcx > Drop for OpaqueTypeStorage < 'tcx > { fn drop (& mut self) { if ! self . is_empty () { ty :: tls :: with (| tcx | tcx . dcx () . delayed_bug (format ! ("{:?}" , self . opaque_types))) ; } } }
    };
}

impl_76!();