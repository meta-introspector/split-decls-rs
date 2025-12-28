macro_rules! __declare_internal_bitflags {
    () => {
        # [doc = " Declare the `bitflags`-facing bitflags struct."] # [doc = ""] # [doc = " This type is part of the `bitflags` crate's public API, but not part of the user's."] # [macro_export] # [doc (hidden)] macro_rules ! __declare_internal_bitflags { ($ vis : vis struct $ InternalBitFlags : ident : $ T : ty) => { # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] $ vis struct $ InternalBitFlags ($ T) ; } ; }
    };
}

__declare_internal_bitflags!()