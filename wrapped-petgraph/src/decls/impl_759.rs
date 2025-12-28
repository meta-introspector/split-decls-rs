macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_759 {
    () => {
        deps!();
        # [doc = " Deref allows transparent access to all shared reference (read-only)"] # [doc = " functionality in the underlying graph."] impl < G > Deref for Frozen < '_ , G > { type Target = G ; fn deref (& self) -> & G { self . 0 } }
    };
}

impl_759!();