macro_rules! deps {
    () => {
        Error!();
        Public!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < V : Version > Purpose < V > for Public { fn validate_header (token : & str) -> Result < () , Error > { if token . is_empty () || ! token . starts_with (V :: PUBLIC_HEADER) { return Err (Error :: TokenFormat) ; } Ok (()) } fn validate_token_message_len (message : & [u8]) -> Result < () , Error > { if message . len () <= V :: PUBLIC_SIG { return Err (Error :: TokenFormat) ; } Ok (()) } fn parse_raw_payload (message : & [u8]) -> & [u8] { debug_assert ! (message . len () > V :: PUBLIC_SIG) ; & message [.. message . len () - V :: PUBLIC_SIG] } }
    };
}

impl_124!();