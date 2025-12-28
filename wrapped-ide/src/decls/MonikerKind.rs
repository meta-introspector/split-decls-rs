macro_rules! MonikerKind {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum MonikerKind { Import , Export , }
    };
}

MonikerKind!();