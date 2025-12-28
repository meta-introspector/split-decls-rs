macro_rules! Delta {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) enum Delta < T > { Add (T) , Sub (T) , }
    };
}

Delta!()