macro_rules! property_values {
    () => {
        # [cfg (any (feature = "unicode-age" , feature = "unicode-bool" , feature = "unicode-gencat" , feature = "unicode-perl" , feature = "unicode-script" , feature = "unicode-segment" ,))] pub mod property_values ;
    };
}

property_values!()