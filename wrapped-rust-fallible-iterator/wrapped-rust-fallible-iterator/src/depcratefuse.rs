// Generated macro for Fuse (struct)
macro_rules! DepcrateFuse {
() => {
// Module: crate
// Provides: {"Fuse"}
// Dependencies: {}
# [doc = " An iterator that yields `Ok(None)` forever after the underlying iterator"] # [doc = " yields `Ok(None)` once."] # [derive (Clone , Debug)] pub struct Fuse < I > { it : I , done : bool , }
};
}
