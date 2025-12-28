macro_rules! FilterMap {
    () => {
        # [doc = " An iterator which both filters and maps the values of the underlying"] # [doc = " iterator."] # [derive (Clone , Debug)] pub struct FilterMap < I , F > { it : I , f : F , }
    };
}

FilterMap!()