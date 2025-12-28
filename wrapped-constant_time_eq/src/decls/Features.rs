macro_rules! Features {
    () => {
        # [doc = " Describes whether `FEAT_DIT` and `FEAT_SB` are known to be implemented."] # [repr (u8)] # [derive (Clone , Copy)] enum Features { # [allow (dead_code)] Neither = 1 , # [allow (dead_code)] DitOnly = 2 , DitSb = 3 , }
    };
}

Features!();