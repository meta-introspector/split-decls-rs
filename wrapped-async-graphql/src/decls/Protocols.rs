macro_rules! Protocols {
    () => {
        # [doc = " Specification of which GraphQL Over WebSockets protocol is being utilized"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] pub enum Protocols { # [doc = " [subscriptions-transport-ws protocol](https://github.com/apollographql/subscriptions-transport-ws/blob/master/PROTOCOL.md)."] SubscriptionsTransportWS , # [doc = " [graphql-ws protocol](https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md)."] GraphQLWS , }
    };
}

Protocols!();