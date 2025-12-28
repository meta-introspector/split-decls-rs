macro_rules! Fuse {
    () => {
        # [doc = " An iterator that yields `Ok(None)` forever after the underlying iterator"] # [doc = " yields `Ok(None)` once."] # [derive (Clone , Debug)] pub struct Fuse < I > { it : I , done : bool , }
    };
}

Fuse!();