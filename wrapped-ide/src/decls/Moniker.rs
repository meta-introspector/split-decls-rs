macro_rules! deps {
    () => {
        MonikerIdentifier!();
        MonikerKind!();
        PackageInformation!();
    };
}

macro_rules! Moniker {
    () => {
        deps!();
        # [doc = " Information which uniquely identifies a definition which might be referenceable outside of the"] # [doc = " source file. Visibility declarations do not affect presence."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Moniker { pub identifier : MonikerIdentifier , pub kind : MonikerKind , pub package_information : PackageInformation , }
    };
}

Moniker!();