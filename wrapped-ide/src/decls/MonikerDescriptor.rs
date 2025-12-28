macro_rules! deps {
    () => {
        MonikerDescriptorKind!();
    };
}

macro_rules! MonikerDescriptor {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct MonikerDescriptor { pub name : String , pub desc : MonikerDescriptorKind , }
    };
}

MonikerDescriptor!()