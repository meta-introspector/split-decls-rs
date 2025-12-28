macro_rules! deps {
    () => {
        SourceName!();
    };
}

macro_rules! DataMemberPrefix {
    () => {
        deps!();
        # [doc = " The `<data-member-prefix>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <data-member-prefix> := <member source-name> M"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct DataMemberPrefix (SourceName) ;
    };
}

DataMemberPrefix!();