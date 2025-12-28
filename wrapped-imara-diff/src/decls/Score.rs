macro_rules! Score {
    () => {
        # [derive (PartialEq , Eq , Clone , Copy)] struct Score { indent : i32 , penalty : i32 , }
    };
}

Score!();