macro_rules! RelocationStateV2 {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Copy)] enum RelocationStateV2 { Start , Extra , Page , PageExtra , }
    };
}

RelocationStateV2!()