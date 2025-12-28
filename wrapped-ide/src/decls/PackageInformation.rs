macro_rules! PackageInformation {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct PackageInformation { pub name : String , pub repo : Option < String > , pub version : Option < String > , }
    };
}

PackageInformation!()