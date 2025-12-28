macro_rules! Role {
    () => {
        # [derive (Clone , Debug , PartialEq)] # [doc = " The role for an instance of HPKE mode."] pub enum Role { # [doc = " HPKE instance for encrypting data."] Sender , # [doc = " HPKE instance for decrypting data."] Recipient , }
    };
}

Role!();