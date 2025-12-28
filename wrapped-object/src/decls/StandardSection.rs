macro_rules! deps {
    () => {
        GnuProperty!();
    };
}

macro_rules! StandardSection {
    () => {
        deps!();
        # [doc = " A standard section kind."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [non_exhaustive] pub enum StandardSection { Text , Data , ReadOnlyData , ReadOnlyDataWithRel , ReadOnlyString , UninitializedData , Tls , # [doc = " Zero-fill TLS initializers. Unsupported for COFF."] UninitializedTls , # [doc = " TLS variable structures. Only supported for Mach-O."] TlsVariables , # [doc = " Common data. Only supported for Mach-O."] Common , # [doc = " Notes for GNU properties. Only supported for ELF."] GnuProperty , }
    };
}

StandardSection!();