macro_rules! PerNS {
    () => {
        # [doc = " Just a helper ‒ separate structure for each namespace."] # [derive (Copy , Clone , Default , Debug , HashStable_Generic)] pub struct PerNS < T > { pub value_ns : T , pub type_ns : T , pub macro_ns : T , }
    };
}

PerNS!()