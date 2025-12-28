macro_rules! deps {
    () => {
        QueueView!();
    };
}

macro_rules! Producer {
    () => {
        deps!();
        # [doc = " A producer; it can enqueue items into the queue."] # [doc = ""] # [doc = " **Note:** The producer semantically owns the `tail` pointer of the queue."] pub struct Producer < 'a , T > { rb : & 'a QueueView < T > , }
    };
}

Producer!();