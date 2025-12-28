macro_rules! macro_362 {
    () => {
        # [cfg (feature = "sink")] delegate_all ! (# [doc = " Future for the [`forward`](super::StreamExt::forward) method."] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] Forward < St , Si > (forward :: Forward < St , Si , St :: Item >) : Debug + Future + FusedFuture + New [| x : St , y : Si | forward :: Forward :: new (x , y)] where St : Stream) ;
    };
}

macro_362!()