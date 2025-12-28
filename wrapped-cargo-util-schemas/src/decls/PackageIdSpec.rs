macro_rules! deps {
    () => {
        PartialVersion!();
        SourceKind!();
    };
}

macro_rules! PackageIdSpec {
    () => {
        deps!();
        # [doc = " Some or all of the data required to identify a package:"] # [doc = ""] # [doc = "  1. the package name (a `String`, required)"] # [doc = "  2. the package version (a `Version`, optional)"] # [doc = "  3. the package source (a `Url`, optional)"] # [doc = ""] # [doc = " If any of the optional fields are omitted, then the package ID may be ambiguous, there may be"] # [doc = " more than one package/version/url combo that will match. However, often just the name is"] # [doc = " sufficient to uniquely define a package ID."] # [derive (Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct PackageIdSpec { name : String , version : Option < PartialVersion > , url : Option < Url > , kind : Option < SourceKind > , }
    };
}

PackageIdSpec!()