macro_rules! NonSubstitution {
    () => {
        # [doc = " A handle to a component that is usually substitutable, and lives in the"] # [doc = " substitutions table, but in this particular case does not qualify for"] # [doc = " substitutions."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct NonSubstitution (usize) ;
    };
}

NonSubstitution!()