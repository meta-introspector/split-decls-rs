macro_rules! TryReadyChunksError {
    () => {
        # [doc = " Error indicating, that while chunk was collected inner stream produced an error."] # [doc = ""] # [doc = " Contains all items that were collected before an error occurred, and the stream error itself."] # [derive (PartialEq , Eq)] pub struct TryReadyChunksError < T , E > (pub Vec < T > , pub E) ;
    };
}

TryReadyChunksError!()