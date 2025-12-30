// Generated macro for Connection (struct)
macro_rules! DepcrateConnection {
() => {
// Module: crate
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " Simple [`SubscriptionConnection`] implementation."] # [doc = ""] # [doc = " Resolves `Value<ValuesStream>` into `Stream<Item = ExecutionOutput<S>>` using"] # [doc = " the following logic:"] # [doc = ""] # [doc = " [`Value::Null`] - returns [`Value::Null`] once"] # [doc = " [`Value::Scalar`] - returns `Ok` value or [`Value::Null`] and errors vector"] # [doc = " [`Value::List`] - resolves each stream from the list using current logic and returns"] # [doc = "                   values in the order received"] # [doc = " [`Value::Object`] - waits while each field of the [`Object`] is returned, then yields the whole object"] # [doc = " `Value::Object<Value::Object<_>>` - returns [`Value::Null`] if [`Value::Object`] consists of sub-objects"] pub struct Connection < 'a , S > { stream : Pin < Box < dyn Stream < Item = ExecutionOutput < S > > + Send + 'a > > , }
};
}
