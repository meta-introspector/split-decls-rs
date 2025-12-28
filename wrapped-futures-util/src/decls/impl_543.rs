macro_rules! deps {
    () => {
        Ready!();
        FlowController!();
        FlowStep!();
        Pending!();
        WrappedWaker!();
    };
}

macro_rules! impl_543 {
    () => {
        deps!();
        impl < St , Fc > Stream for FlattenUnorderedWithFlowController < St , Fc > where St : Stream , Fc : FlowController < St :: Item , < St :: Item as Stream > :: Item > , St :: Item : Stream + Unpin , { type Item = < St :: Item as Stream > :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut next_item = None ; let mut need_to_poll_next = NONE ; let mut this = self . as_mut () . project () ; let (mut poll_state_value , state_bomb) = loop { if let Some (value) = this . poll_state . start_polling () { break value ; } } ; unsafe { WrappedWaker :: replace_waker (this . stream_waker , cx) ; WrappedWaker :: replace_waker (this . inner_streams_waker , cx) } ; if poll_state_value & NEED_TO_POLL_STREAM != NONE { let mut stream_waker = None ; loop { if this . is_exceeded_limit () || * this . is_stream_done { if ! * this . is_stream_done { need_to_poll_next |= NEED_TO_POLL_STREAM ; } break ; } else { let mut cx = Context :: from_waker (stream_waker . get_or_insert_with (| | waker (this . stream_waker . clone ())) ,) ; match this . stream . as_mut () . poll_next (& mut cx) { Poll :: Ready (Some (item)) => { let next_item_fut = match Fc :: next_step (item) { FlowStep :: Return (item) => { need_to_poll_next |= NEED_TO_POLL_STREAM | (poll_state_value & NEED_TO_POLL_INNER_STREAMS) ; poll_state_value &= ! NEED_TO_POLL_INNER_STREAMS ; next_item = Some (item) ; break ; } FlowStep :: Continue (inner_stream) => { PollStreamFut :: new (inner_stream) } } ; this . inner_streams . as_mut () . push (next_item_fut) ; poll_state_value |= NEED_TO_POLL_INNER_STREAMS ; } Poll :: Ready (None) => { * this . is_stream_done = true ; } Poll :: Pending => { break ; } } } } } if poll_state_value & NEED_TO_POLL_INNER_STREAMS != NONE { let inner_streams_waker = waker (this . inner_streams_waker . clone ()) ; let mut cx = Context :: from_waker (& inner_streams_waker) ; match this . inner_streams . as_mut () . poll_next (& mut cx) { Poll :: Ready (Some (Some ((item , next_item_fut)))) => { this . inner_streams . as_mut () . push (next_item_fut) ; next_item = Some (item) ; need_to_poll_next |= NEED_TO_POLL_INNER_STREAMS ; } Poll :: Ready (Some (None)) => { need_to_poll_next |= NEED_TO_POLL_INNER_STREAMS ; } _ => { } } } state_bomb . deactivate () ; let mut force_wake = need_to_poll_next & NEED_TO_POLL_STREAM != NONE && ! this . is_exceeded_limit () || need_to_poll_next & NEED_TO_POLL_INNER_STREAMS != NONE ; poll_state_value = this . poll_state . stop_polling (need_to_poll_next , force_wake) ; force_wake |= poll_state_value & NEED_TO_POLL_ALL != NONE ; let is_done = * this . is_stream_done && this . inner_streams . is_empty () ; if next_item . is_some () || is_done { Poll :: Ready (next_item) } else { if force_wake { cx . waker () . wake_by_ref () ; } Poll :: Pending } } }
    };
}

impl_543!();