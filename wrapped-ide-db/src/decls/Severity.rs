macro_rules! Severity {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum Severity { Error , Warning , WeakWarning , Allow , }
    };
}

Severity!()