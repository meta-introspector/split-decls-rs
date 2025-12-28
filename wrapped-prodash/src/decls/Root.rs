macro_rules! deps {
    () => {
        Message!();
        Key!();
        WeakRoot!();
        Task!();
        MessageCopyState!();
    };
}

macro_rules! Root {
    () => {
        deps!();
        # [doc = " The top level of a progress task hierarchy, with `progress::Task`s identified with `progress::Key`s"] pub trait Root { # [doc = " The type implementing the `WeakRoot` trait"] type WeakRoot : WeakRoot ; # [doc = " Returns the maximum amount of messages we can keep before overwriting older ones."] fn messages_capacity (& self) -> usize ; # [doc = " Returns the current amount of tasks underneath the root, transitively."] # [doc = " **Note** that this is at most a guess as tasks can be added and removed in parallel."] fn num_tasks (& self) -> usize ; # [doc = " Copy the entire progress tree into the given `out` vector, so that"] # [doc = " it can be traversed from beginning to end in order of hierarchy."] # [doc = " The `out` vec will be cleared automatically."] fn sorted_snapshot (& self , out : & mut Vec < (progress :: Key , progress :: Task) >) ; # [doc = " Copy all messages from the internal ring buffer into the given `out`"] # [doc = " vector. Messages are ordered from oldest to newest."] fn copy_messages (& self , out : & mut Vec < Message >) ; # [doc = " Copy only new messages from the internal ring buffer into the given `out`"] # [doc = " vector. Messages are ordered from oldest to newest."] fn copy_new_messages (& self , out : & mut Vec < Message > , prev : Option < MessageCopyState >) -> MessageCopyState ; # [doc = " Similar to `Arc::downgrade()`"] fn downgrade (& self) -> Self :: WeakRoot ; }
    };
}

Root!()