macro_rules! PropertyValues {
    () => {
        # [doc = " A mapping of property values for a specific property."] # [doc = ""] # [doc = " The first element of each tuple is a normalized property value while the"] # [doc = " second element of each tuple is the corresponding canonical property"] # [doc = " value."] type PropertyValues = & 'static [(& 'static str , & 'static str)] ;
    };
}

PropertyValues!();