macro_rules! AllowSelfProjection {
    () => {
        # [derive (Clone , Copy)] enum AllowSelfProjection { Yes , No , }
    };
}

AllowSelfProjection!();