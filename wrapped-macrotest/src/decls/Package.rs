macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! Package {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub struct Package { pub name : String , pub version : String , pub publish : bool , pub edition : Edition , }
    };
}

Package!()