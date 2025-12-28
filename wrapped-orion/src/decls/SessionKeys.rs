macro_rules! SessionKeys {
    () => {
        # [allow (clippy :: derive_partial_eq_without_eq)] # [derive (Debug , PartialEq)] # [doc = " A set of shared secrets for either transmitting to this entity or send to another party."] pub struct SessionKeys { rx : SecretKey , tx : SecretKey , }
    };
}

SessionKeys!();