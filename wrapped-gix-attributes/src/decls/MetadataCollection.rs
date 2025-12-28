macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! MetadataCollection {
    () => {
        deps!();
        # [doc = " A utility type to collect metadata for each attribute, unified by its name."] # [derive (Clone , Debug , Default)] pub struct MetadataCollection { # [doc = " A mapping of an attribute or macro name to its order, that is the time when it was *first* seen."] # [doc = ""] # [doc = " This is the inverse of the order attributes are searched."] name_to_meta : HashMap < KString , Metadata > , }
    };
}

MetadataCollection!();