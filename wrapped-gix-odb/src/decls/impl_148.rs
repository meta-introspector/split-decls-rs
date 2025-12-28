macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < T > Proxy < T > { # [doc = " Create a new instance using `odb` as actual object provider, with an empty in-memory store for"] # [doc = " objects that are to be written."] # [doc = " Use `object_hash` to determine the kind of hash to produce when writing new objects."] pub fn new (odb : T , object_hash : gix_hash :: Kind) -> Proxy < T > { Proxy { inner : odb , object_hash , memory : Some (Default :: default ()) , } } # [doc = " Turn ourselves into our inner object database, while deallocating objects stored in memory."] pub fn into_inner (self) -> T { self . inner } # [doc = " Strip object memory off this instance, which means that writes will go through to the inner object database"] # [doc = " right away."] # [doc = " This mode makes the proxy fully transparent."] pub fn with_write_passthrough (mut self) -> Self { self . memory . take () ; self } }
    };
}

impl_148!();