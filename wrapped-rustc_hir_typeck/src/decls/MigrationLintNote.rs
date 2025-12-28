macro_rules! deps {
    () => {
        UpvarMigrationInfo!();
        MigrationWarningReason!();
    };
}

macro_rules! MigrationLintNote {
    () => {
        deps!();
        # [doc = " Intermediate format to store information needed to generate a note in the migration lint."] struct MigrationLintNote { captures_info : UpvarMigrationInfo , # [doc = " reasons why migration is needed for this capture"] reason : MigrationWarningReason , }
    };
}

MigrationLintNote!();