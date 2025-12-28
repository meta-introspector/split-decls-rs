macro_rules! deps {
    () => {
        MonikerDescriptor!();
    };
}

macro_rules! MonikerIdentifier {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct MonikerIdentifier { pub crate_name : String , pub description : Vec < MonikerDescriptor > , }
    };
}

MonikerIdentifier!();