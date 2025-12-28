macro_rules! deps {
    () => {
        Enumerate!();
        Take!();
        FromConcurrentStream!();
        Consumer!();
        Limit!();
        Map!();
        ForEachConsumer!();
        TryForEachConsumer!();
    };
}

macro_rules! ConcurrentStream {
    () => {
        deps!();
        # [doc = " Concurrently operate over items in a stream"] # [allow (async_fn_in_trait)] pub trait ConcurrentStream { # [doc = " Which item will we be yielding?"] type Item ; # [doc = " What's the type of the future containing our items?"] type Future : Future < Output = Self :: Item > ; # [doc = " Internal method used to define the behavior of this concurrent iterator."] # [doc = " You should not need to call this directly. This method causes the"] # [doc = " iterator self to start producing items and to feed them to the consumer"] # [doc = " consumer one by one."] async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > ; # [doc = " How much concurrency should we apply?"] fn concurrency_limit (& self) -> Option < NonZeroUsize > ; # [doc = " How many items could we potentially end up returning?"] fn size_hint (& self) -> (usize , Option < usize >) { (0 , None) } # [doc = " Creates a stream which gives the current iteration count as well as"] # [doc = " the next value."] # [doc = ""] # [doc = " The value is determined by the moment the future is created, not the"] # [doc = " moment the future is evaluated."] fn enumerate (self) -> Enumerate < Self > where Self : Sized , { Enumerate :: new (self) } # [doc = " Obtain a simple pass-through adapter."] fn limit (self , limit : Option < NonZeroUsize >) -> Limit < Self > where Self : Sized , { Limit :: new (self , limit) } # [doc = " Creates a stream that yields the first `n` elements, or fewer if the"] # [doc = " underlying iterator ends sooner."] fn take (self , limit : usize) -> Take < Self > where Self : Sized , { Take :: new (self , limit) } # [doc = " Convert items from one type into another"] fn map < F , FutB , B > (self , f : F) -> Map < Self , F , Self :: Future , Self :: Item , FutB , B > where Self : Sized , F : Fn (Self :: Item) -> FutB , F : Clone , FutB : Future < Output = B > , { Map :: new (self , f) } # [doc = " Iterate over each item concurrently"] async fn for_each < F , Fut > (self , f : F) where Self : Sized , F : Fn (Self :: Item) -> Fut , F : Clone , Fut : Future < Output = () > , { let limit = self . concurrency_limit () ; self . drive (ForEachConsumer :: new (limit , f)) . await } # [doc = " Iterate over each item concurrently, short-circuit on error."] # [doc = ""] # [doc = " If an error is returned this will cancel all other futures."] async fn try_for_each < F , Fut , E > (self , f : F) -> Result < () , E > where Self : Sized , F : Fn (Self :: Item) -> Fut , F : Clone , Fut : Future < Output = Result < () , E > > , { let limit = self . concurrency_limit () ; self . drive (TryForEachConsumer :: new (limit , f)) . await } # [doc = " Transforms an iterator into a collection."] async fn collect < B > (self) -> B where B : FromConcurrentStream < Self :: Item > , Self : Sized , { B :: from_concurrent_stream (self) . await } }
    };
}

ConcurrentStream!()