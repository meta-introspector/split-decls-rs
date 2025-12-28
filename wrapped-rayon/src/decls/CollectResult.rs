macro_rules! deps {
    () => {
        SendPtr!();
        CollectReducer!();
    };
}

macro_rules! CollectResult {
    () => {
        deps!();
        # [doc = " CollectResult represents an initialized part of the target slice."] # [doc = ""] # [doc = " This is a proxy owner of the elements in the slice; when it drops,"] # [doc = " the elements will be dropped, unless its ownership is released before then."] # [must_use] pub (super) struct CollectResult < 'c , T > { # [doc = " This pointer and length has the same representation as a slice,"] # [doc = " but retains the provenance of the entire array so that we can merge"] # [doc = " these regions together in `CollectReducer`."] start : SendPtr < T > , total_len : usize , # [doc = " The current initialized length after `start`"] initialized_len : usize , # [doc = " Lifetime invariance guarantees that the data flows from consumer to result,"] # [doc = " especially for the `scope_fn` callback in `Collect::with_consumer`."] invariant_lifetime : PhantomData < & 'c mut & 'c mut [T] > , }
    };
}

CollectResult!()