macro_rules! deps {
    () => {
        PropertyValues!();
    };
}

macro_rules! canonical_value {
    () => {
        deps!();
        # [doc = " Find the canonical property value for the given normalized property"] # [doc = " value."] # [doc = ""] # [doc = " The given property values should correspond to the values for the property"] # [doc = " under question, which can be found using `property_values`."] # [doc = ""] # [doc = " If no such property value exists, then `None` is returned."] # [doc = ""] # [doc = " The normalized property value must have been normalized according to"] # [doc = " UAX44 LM3, which can be done using `symbolic_name_normalize`."] fn canonical_value (vals : PropertyValues , normalized_value : & str ,) -> Option < & 'static str > { vals . binary_search_by_key (& normalized_value , | & (n , _) | n) . ok () . map (| i | vals [i] . 1) }
    };
}

canonical_value!();