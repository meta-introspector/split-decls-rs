macro_rules! SALT_LENGTH {
    () => {
        # [doc = " The length of the salt used for password hashing."] pub const SALT_LENGTH : usize = 16 ;
    };
}

SALT_LENGTH!()