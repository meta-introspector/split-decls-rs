// Generated macro for create_and_execute (function)
macro_rules! Depcrate_tests_subscriptionscreate_and_execute {
() => {
// Module: crate::tests::subscriptions
// Provides: {"create_and_execute"}
// Dependencies: {}
# [doc = " Create all variables, execute subscription"] # [doc = " and collect returned iterators."] # [doc = " Panics if query is invalid (GraphQLError is returned)"] fn create_and_execute (query : String ,) -> Result < (Vec < String > , Vec < Vec < Result < Value < DefaultScalarValue > , ExecutionError < DefaultScalarValue > > > > ,) , Vec < ExecutionError < DefaultScalarValue > > , > { let request = GraphQLRequest :: new (query , None , None) ; let root_node = Schema :: new (MyQuery , EmptyMutation :: new () , MySubscription) ; let context = MyContext (2) ; let response = run (crate :: http :: resolve_into_stream (& request , & root_node , & context ,)) ; assert ! (response . is_ok ()) ; let (values , errors) = response . unwrap () ; if ! errors . is_empty () { return Err (errors) ; } let response_value_object = match values { Value :: Object (o) => Some (o) , _ => None , } ; assert ! (response_value_object . is_some ()) ; let response_returned_object = response_value_object . unwrap () ; let fields = response_returned_object . into_iter () ; let mut names = vec ! [] ; let mut collected_values = vec ! [] ; for (name , stream_val) in fields { names . push (name . clone ()) ; match stream_val { Value :: Scalar (stream) => { let collected = run (stream . collect :: < Vec < _ > > ()) ; collected_values . push (collected) ; } _ => unreachable ! () , } } Ok ((names , collected_values)) }
};
}
