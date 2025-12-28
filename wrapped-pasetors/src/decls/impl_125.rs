macro_rules! deps {
    () => {
        Local!();
        Error!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < V : Version > Purpose < V > for Local { fn validate_header (token : & str) -> Result < () , Error > { if token . is_empty () || ! token . starts_with (V :: LOCAL_HEADER) { return Err (Error :: TokenFormat) ; } Ok (()) } fn validate_token_message_len (message : & [u8]) -> Result < () , Error > { if message . len () <= V :: LOCAL_NONCE + V :: LOCAL_TAG { return Err (Error :: TokenFormat) ; } Ok (()) } fn parse_raw_payload (message : & [u8]) -> & [u8] { debug_assert ! (message . len () > V :: LOCAL_TAG + V :: LOCAL_NONCE) ; & message [V :: LOCAL_NONCE .. message . len () - V :: LOCAL_TAG] } }
    };
}

impl_125!();