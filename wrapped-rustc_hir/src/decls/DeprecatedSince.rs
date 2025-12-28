macro_rules! deps {
    () => {
        RustcVersion!();
        Deprecation!();
        PrintAttribute!();
    };
}

macro_rules! DeprecatedSince {
    () => {
        deps!();
        # [doc = " Release in which an API is deprecated."] # [derive (Copy , Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub enum DeprecatedSince { RustcVersion (RustcVersion) , # [doc = " Deprecated in the future (\"to be determined\")."] Future , # [doc = " `feature(staged_api)` is off. Deprecation versions outside the standard"] # [doc = " library are allowed to be arbitrary strings, for better or worse."] NonStandard (Symbol) , # [doc = " Deprecation version is unspecified but optional."] Unspecified , # [doc = " Failed to parse a deprecation version, or the deprecation version is"] # [doc = " unspecified and required. An error has already been emitted."] Err , }
    };
}

DeprecatedSince!();