macro_rules! macro_629 {
    () => {
        # [cfg (feature = "sink")] delegate_all ! (# [doc = " Future for the [`try_forward`](super::TryStreamExt::try_forward) method."] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] TryForward < St , Si > (try_forward :: TryForward < St , Si , St :: Ok >) : Debug + Future + FusedFuture + New [| x : St , y : Si | try_forward :: TryForward :: new (x , y)] where St : TryStream) ;
    };
}

macro_629!();