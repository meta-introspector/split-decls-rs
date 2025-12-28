macro_rules! deps {
    () => {
        Storage!();
    };
}

macro_rules! Proxy {
    () => {
        deps!();
        # [doc = " An object database to read from any implementation but write to memory."] # [doc = " Previously written objects can be returned from memory upon query, which makes the view of objects consistent."] # [doc = " In-Memory objects can be disabled by [taking out its storage](Proxy::take_object_memory). From there in-memory"] # [doc = " object can also be persisted one by one."] # [doc = ""] # [doc = " It's possible to turn off the memory by removing it from the instance."] pub struct Proxy < T > { # [doc = " The actual odb implementation"] inner : T , # [doc = " The kind of hash to produce when writing new objects."] object_hash : gix_hash :: Kind , # [doc = " The storage for in-memory objects."] # [doc = " If `None`, the proxy will always read from and write-through to `inner`."] memory : Option < RefCell < Storage > > , }
    };
}

Proxy!();