macro_rules! deps {
    () => {
        ReadState!();
    };
}

macro_rules! macro_719 {
    () => {
        deps!();
        pin_project ! { # [doc = " Reader for the [`into_async_read`](super::TryStreamExt::into_async_read) method."] # [derive (Debug)] # [must_use = "readers do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "io")))] pub struct IntoAsyncRead < St > where St : TryStream < Error = Error >, St :: Ok : AsRef < [u8] >, { # [pin] stream : St , state : ReadState < St :: Ok >, } }
    };
}

macro_719!()