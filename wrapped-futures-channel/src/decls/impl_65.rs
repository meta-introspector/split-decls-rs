macro_rules! deps {
    () => {
        UnboundedInner!();
        UnboundedSenderInner!();
        SendError!();
        SendErrorKind!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T > UnboundedSenderInner < T > { fn poll_ready_nb (& self) -> Poll < Result < () , SendError > > { let state = decode_state (self . inner . state . load (SeqCst)) ; if state . is_open { Poll :: Ready (Ok (())) } else { Poll :: Ready (Err (SendError { kind : SendErrorKind :: Disconnected })) } } fn queue_push_and_signal (& self , msg : T) { self . inner . message_queue . push (msg) ; self . inner . recv_task . wake () ; } fn inc_num_messages (& self) -> Option < usize > { let mut curr = self . inner . state . load (SeqCst) ; loop { let mut state = decode_state (curr) ; if ! state . is_open { return None ; } assert ! (state . num_messages < MAX_CAPACITY , "buffer space \
                    exhausted; sending this messages would overflow the state") ; state . num_messages += 1 ; let next = encode_state (& state) ; match self . inner . state . compare_exchange (curr , next , SeqCst , SeqCst) { Ok (_) => return Some (state . num_messages) , Err (actual) => curr = actual , } } } # [doc = " Returns whether the senders send to the same receiver."] fn same_receiver (& self , other : & Self) -> bool { Arc :: ptr_eq (& self . inner , & other . inner) } # [doc = " Returns whether the sender send to this receiver."] fn is_connected_to (& self , inner : & Arc < UnboundedInner < T > >) -> bool { Arc :: ptr_eq (& self . inner , inner) } # [doc = " Returns pointer to the Arc containing sender"] # [doc = ""] # [doc = " The returned pointer is not referenced and should be only used for hashing!"] fn ptr (& self) -> * const UnboundedInner < T > { & * self . inner } # [doc = " Returns whether this channel is closed without needing a context."] fn is_closed (& self) -> bool { ! decode_state (self . inner . state . load (SeqCst)) . is_open } # [doc = " Closes this channel from the sender side, preventing any new messages."] fn close_channel (& self) { self . inner . set_closed () ; self . inner . recv_task . wake () ; } }
    };
}

impl_65!();