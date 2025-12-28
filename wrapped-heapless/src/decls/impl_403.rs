macro_rules! deps {
    () => {
        QueueView!();
        Storage!();
        Cell!();
        AtomicTargetSize!();
        Queue!();
        UintSize!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < T , const N : usize > Queue < T , N > { # [deprecated (note = "See the documentation of Queue::new() for more information: https://docs.rs/heapless/latest/heapless/mpmc/type.Queue.html#method.new")] # [doc = " Creates an empty queue."] # [doc = ""] # [doc = " # Deprecation"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = " The current implementation of `mpmc` is marked as deprecated due to not being truly"] # [doc = " lock-free </div>"] # [doc = ""] # [doc = " If a thread is parked, or pre-empted for a long time by an higher-priority task"] # [doc = " during an `enqueue` or `dequeue` operation, it is possible that the queue ends-up"] # [doc = " in a state were no other task can successfully enqueue or dequeue items from it"] # [doc = " until the pre-empted task can finish its operation."] # [doc = ""] # [doc = " In that case, [`enqueue`](QueueInner::dequeue) and [`dequeue`](QueueInner::enqueue)"] # [doc = " will return an error, but will not panic or reach undefined behaviour"] # [doc = ""] # [doc = " This makes `mpmc` unsuitable for some use cases such as using it as a pool of objects."] # [doc = ""] # [doc = " ## When can this queue be used?"] # [doc = ""] # [doc = " This queue should be used for cross-task communication only when items sent over the queue"] # [doc = " can be dropped in case of concurrent operations, or when it is possible to retry"] # [doc = " the dequeue/enqueue operation after other tasks have had the opportunity to make progress."] # [doc = ""] # [doc = " In that case you can safely ignore the warnings using `#[expect(deprecated)]`"] # [doc = " when `new` is called"] # [doc = ""] # [doc = " For more information, and possible alternative, please see"] # [doc = " <https://github.com/rust-embedded/heapless/issues/583>"] pub const fn new () -> Self { const { assert ! (N > 1) ; assert ! (N . is_power_of_two ()) ; assert ! (N < UintSize :: MAX as usize) ; } let mut cell_count = 0 ; let mut result_cells : [Cell < T > ; N] = [const { Cell :: new (0) } ; N] ; while cell_count != N { result_cells [cell_count] = Cell :: new (cell_count) ; cell_count += 1 ; } Self { buffer : UnsafeCell :: new (result_cells) , dequeue_pos : AtomicTargetSize :: new (0) , enqueue_pos : AtomicTargetSize :: new (0) , } } # [doc = " Used in `Storage` implementation."] pub (crate) fn as_view_private (& self) -> & QueueView < T > { self } # [doc = " Used in `Storage` implementation."] pub (crate) fn as_view_mut_private (& mut self) -> & mut QueueView < T > { self } }
    };
}

impl_403!();