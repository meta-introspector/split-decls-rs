macro_rules! MonikerDescriptorKind {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum MonikerDescriptorKind { Namespace , Type , Term , Method , TypeParameter , Parameter , Macro , Meta , }
    };
}

MonikerDescriptorKind!()