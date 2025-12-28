macro_rules! deps {
    () => {
        Result!();
        Error!();
        Protocols!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl std :: str :: FromStr for Protocols { type Err = Error ; fn from_str (protocol : & str) -> Result < Self , Self :: Err > { if protocol . eq_ignore_ascii_case ("graphql-ws") { Ok (Protocols :: SubscriptionsTransportWS) } else if protocol . eq_ignore_ascii_case ("graphql-transport-ws") { Ok (Protocols :: GraphQLWS) } else { Err (Error :: new (format ! ("Unsupported Sec-WebSocket-Protocol: {}" , protocol))) } } }
    };
}

impl_661!();