macro_rules! LazyProperty {
    () => {
        # [doc = " A type signaling that a value is either computed, or is available for computation."] # [derive (Clone , Debug , Default , UpmapFromRaFixture)] pub enum LazyProperty < T > { Computed (T) , # [default] Lazy , }
    };
}

LazyProperty!()