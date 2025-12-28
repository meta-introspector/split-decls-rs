macro_rules! deps {
    () => {
        IndexerProgress!();
    };
}

macro_rules! TransportProgress {
    () => {
        deps!();
        # [doc = " Callback to be invoked while a transfer is in progress."] # [doc = ""] # [doc = " This callback will be periodically called with updates to the progress of"] # [doc = " the transfer so far. The return value indicates whether the transfer should"] # [doc = " continue. A return value of `false` will cancel the transfer."] # [doc = ""] # [doc = " * `progress` - the progress being made so far."] # [deprecated (since = "0.11.0" , note = "renamed to `IndexerProgress` to match upstream")] # [allow (dead_code)] pub type TransportProgress < 'a > = IndexerProgress < 'a > ;
    };
}

TransportProgress!()