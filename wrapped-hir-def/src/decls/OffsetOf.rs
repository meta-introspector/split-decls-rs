macro_rules! OffsetOf {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct OffsetOf { pub container : TypeRefId , pub fields : Box < [Name] > , }
    };
}

OffsetOf!()