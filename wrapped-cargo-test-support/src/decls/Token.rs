macro_rules! Token {
    () => {
        # [doc = " Auth-token for publishing, see [`RegistryBuilder::token`]"] # [derive (Clone)] pub enum Token { Plaintext (String) , Keys (String , Option < String >) , }
    };
}

Token!()