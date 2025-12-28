macro_rules! deps {
    () => {
        WriteBufferManagerWrapper!();
    };
}

macro_rules! WriteBufferManager {
    () => {
        deps!();
        # [derive (Clone)] pub struct WriteBufferManager (pub (crate) Arc < WriteBufferManagerWrapper >) ;
    };
}

WriteBufferManager!()