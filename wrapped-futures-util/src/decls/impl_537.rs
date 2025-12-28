macro_rules! deps {
    () => {
        FuturesUnordered!();
        SharedPollState!();
        FlowController!();
        WrappedWaker!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl < St , Fc > FlattenUnorderedWithFlowController < St , Fc > where St : Stream , Fc : FlowController < St :: Item , < St :: Item as Stream > :: Item > , St :: Item : Stream + Unpin , { pub (crate) fn new (stream : St , limit : Option < usize >) -> Self { let poll_state = SharedPollState :: new (NEED_TO_POLL_STREAM) ; Self { inner_streams : FuturesUnordered :: new () , stream , is_stream_done : false , limit : limit . and_then (NonZeroUsize :: new) , inner_streams_waker : Arc :: new (WrappedWaker { inner_waker : UnsafeCell :: new (None) , poll_state : poll_state . clone () , need_to_poll : NEED_TO_POLL_INNER_STREAMS , }) , stream_waker : Arc :: new (WrappedWaker { inner_waker : UnsafeCell :: new (None) , poll_state : poll_state . clone () , need_to_poll : NEED_TO_POLL_STREAM , }) , poll_state , flow_controller : PhantomData , } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_537!();