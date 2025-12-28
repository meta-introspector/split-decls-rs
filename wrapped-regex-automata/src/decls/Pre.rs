macro_rules! deps {
    () => {
        GroupInfo!();
    };
}

macro_rules! Pre {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct Pre < P > { pre : P , group_info : GroupInfo , }
    };
}

Pre!()