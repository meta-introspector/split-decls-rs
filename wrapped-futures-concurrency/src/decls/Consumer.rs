macro_rules! deps {
    () => {
        ConsumerState!();
    };
}

macro_rules! Consumer {
    () => {
        deps!();
        # [doc = " Describes a type which can receive data."] # [doc = ""] # [doc = " # Type Generics"] # [doc = " - `Item` in this context means the item that it will  repeatedly receive."] # [doc = " - `Future` in this context refers to the future type repeatedly submitted to it."] # [allow (async_fn_in_trait)] pub trait Consumer < Item , Fut > where Fut : Future < Output = Item > , { # [doc = " What is the type of the item we're returning when completed?"] type Output ; # [doc = " Send an item down to the next step in the processing queue."] async fn send (self : Pin < & mut Self > , fut : Fut) -> ConsumerState ; # [doc = " Make progress on the consumer while doing something else."] # [doc = ""] # [doc = " It should always be possible to drop the future returned by this"] # [doc = " function. This is solely intended to keep work going on the `Consumer`"] # [doc = " while doing e.g. waiting for new futures from a stream."] async fn progress (self : Pin < & mut Self >) -> ConsumerState ; # [doc = " We have no more data left to send to the `Consumer`; wait for its"] # [doc = " output."] async fn flush (self : Pin < & mut Self >) -> Self :: Output ; }
    };
}

Consumer!();