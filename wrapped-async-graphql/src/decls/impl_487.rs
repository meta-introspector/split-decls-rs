macro_rules! deps {
    () => {
        Data!();
        Schema!();
        Response!();
        Request!();
        Executor!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl Executor for Schema { async fn execute (& self , request : Request) -> Response { Schema :: execute (self , request) . await } fn execute_stream (& self , request : Request , session_data : Option < Arc < Data > > ,) -> BoxStream < 'static , Response > { Schema :: execute_stream_with_session_data (self , request , session_data . unwrap_or_default ()) . boxed () } }
    };
}

impl_487!();