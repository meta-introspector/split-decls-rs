macro_rules! deps {
    () => {
        Receiver!();
        Lock!();
        Sender!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        # [doc = " Internal state of the `Receiver`/`Sender` pair above. This is all used as"] # [doc = " the internal synchronization between the two for send/recv operations."] struct Inner < T > { # [doc = " Indicates whether this oneshot is complete yet. This is filled in both"] # [doc = " by `Sender::drop` and by `Receiver::drop`, and both sides interpret it"] # [doc = " appropriately."] # [doc = ""] # [doc = " For `Receiver`, if this is `true`, then it's guaranteed that `data` is"] # [doc = " unlocked and ready to be inspected."] # [doc = ""] # [doc = " For `Sender` if this is `true` then the oneshot has gone away and it"] # [doc = " can return ready from `poll_canceled`."] complete : AtomicBool , # [doc = " The actual data being transferred as part of this `Receiver`. This is"] # [doc = " filled in by `Sender::complete` and read by `Receiver::poll`."] # [doc = ""] # [doc = " Note that this is protected by `Lock`, but it is in theory safe to"] # [doc = " replace with an `UnsafeCell` as it's actually protected by `complete`"] # [doc = " above. I wouldn't recommend doing this, however, unless someone is"] # [doc = " supremely confident in the various atomic orderings here and there."] data : Lock < Option < T > > , # [doc = " Field to store the task which is blocked in `Receiver::poll`."] # [doc = ""] # [doc = " This is filled in when a oneshot is polled but not ready yet. Note that"] # [doc = " the `Lock` here, unlike in `data` above, is important to resolve races."] # [doc = " Both the `Receiver` and the `Sender` halves understand that if they"] # [doc = " can't acquire the lock then some important interference is happening."] rx_task : Lock < Option < Waker > > , # [doc = " Like `rx_task` above, except for the task blocked in"] # [doc = " `Sender::poll_canceled`. Additionally, `Lock` cannot be `UnsafeCell`."] tx_task : Lock < Option < Waker > > , }
    };
}

Inner!()