macro_rules! deps {
    () => {
        Savepoint!();
        Transaction!();
    };
}

macro_rules! DropBehavior {
    () => {
        deps!();
        # [doc = " Options for how a Transaction or Savepoint should behave when it is dropped."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum DropBehavior { # [doc = " Roll back the changes. This is the default."] Rollback , # [doc = " Commit the changes."] Commit , # [doc = " Do not commit or roll back changes - this will leave the transaction or"] # [doc = " savepoint open, so should be used with care."] Ignore , # [doc = " Panic. Used to enforce intentional behavior during development."] Panic , }
    };
}

DropBehavior!()