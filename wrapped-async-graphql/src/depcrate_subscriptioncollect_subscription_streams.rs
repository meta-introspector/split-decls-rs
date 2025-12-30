// Generated macro for collect_subscription_streams (function)
macro_rules! Depcrate_subscriptioncollect_subscription_streams {
() => {
// Module: crate::subscription
// Provides: {"collect_subscription_streams"}
// Dependencies: {}
pub (crate) fn collect_subscription_streams < 'a , T : SubscriptionType + 'static > (ctx : & ContextSelectionSet < 'a > , root : & 'a T , streams : & mut Vec < BoxFieldStream < 'a > > ,) -> ServerResult < () > { for selection in & ctx . item . node . items { if let Selection :: Field (field) = & selection . node { streams . push (Box :: pin ({ let ctx = ctx . clone () ; async_stream :: stream ! { let ctx = ctx . with_field (field) ; let field_name = ctx . item . node . response_key () . node . clone () ; let stream = root . create_field_stream (& ctx) ; if let Some (mut stream) = stream { while let Some (resp) = stream . next () . await { yield resp ; } } else { let err = ServerError :: new (format ! (r#"Cannot query field "{}" on type "{}"."# , field_name , T :: type_name ()) , Some (ctx . item . pos)) . with_path (vec ! [PathSegment :: Field (field_name . to_string ())]) ; yield Response :: from_errors (vec ! [err]) ; } } })) } } Ok (()) }
};
}
