macro_rules! MigrationWarningReason {
    () => {
        # [doc = " Reasons that we might issue a migration warning."] # [derive (Clone , Debug , Default , PartialEq , Eq , PartialOrd , Ord , Hash)] struct MigrationWarningReason { # [doc = " When we used to capture `x` in its entirety, we implemented the auto-trait(s)"] # [doc = " in this vec, but now we don't."] auto_traits : Vec < & 'static str > , # [doc = " When we used to capture `x` in its entirety, we would execute some destructors"] # [doc = " at a different time."] drop_order : bool , }
    };
}

MigrationWarningReason!();