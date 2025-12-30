// Generated macro for collect_typename_field (function)
macro_rules! Depcrate_dynamic_resolvecollect_typename_field {
() => {
// Module: crate::dynamic::resolve
// Provides: {"collect_typename_field"}
// Dependencies: {}
fn collect_typename_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , object : & 'a Object , field : & 'a Positioned < Field > ,) { fields . push (async move { Ok ((field . node . response_key () . node . clone () , Value :: from (object . name . as_str ()) ,)) } . boxed () ,) }
};
}
