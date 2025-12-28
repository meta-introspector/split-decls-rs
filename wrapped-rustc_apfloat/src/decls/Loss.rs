macro_rules! Loss {
    () => {
        # [doc = " Enum that represents what fraction of the LSB truncated bits of an fp number"] # [doc = " represent."] # [doc = ""] # [doc = " This essentially combines the roles of guard and sticky bits."] # [must_use] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum Loss { ExactlyZero , LessThanHalf , ExactlyHalf , MoreThanHalf , }
    };
}

Loss!();