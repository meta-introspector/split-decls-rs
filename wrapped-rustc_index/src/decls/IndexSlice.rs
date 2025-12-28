macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! IndexSlice {
    () => {
        deps!();
        # [doc = " A view into contiguous `T`s, indexed by `I` rather than by `usize`."] # [doc = ""] # [doc = " One common pattern you'll see is code that uses [`IndexVec::from_elem`]"] # [doc = " to create the storage needed for a particular \"universe\" (aka the set of all"] # [doc = " the possible keys that need an associated value) then passes that working"] # [doc = " area as `&mut IndexSlice<I, T>` to clarify that nothing will be added nor"] # [doc = " removed during processing (and, as a bonus, to chase fewer pointers)."] # [derive (PartialEq , Eq , Hash)] # [repr (transparent)] pub struct IndexSlice < I : Idx , T > { _marker : PhantomData < fn (& I) > , pub raw : [T] , }
    };
}

IndexSlice!()