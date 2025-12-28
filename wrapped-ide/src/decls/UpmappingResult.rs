macro_rules! UpmappingResult {
    () => {
        # [derive (Debug)] pub struct UpmappingResult < T > { # [doc = " The macro call site."] pub call_site : T , # [doc = " The macro definition site, if relevant."] pub def_site : Option < T > , }
    };
}

UpmappingResult!()