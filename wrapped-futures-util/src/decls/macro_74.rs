macro_rules! deps {
    () => {
        SendMsg!();
        RemoteHandle!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        pin_project ! { # [doc = " A future which sends its output to the corresponding `RemoteHandle`."] # [doc = " Created by [`remote_handle`](crate::future::FutureExt::remote_handle)."] # [must_use = "futures do nothing unless you `.await` or poll them"] # [cfg_attr (docsrs , doc (cfg (feature = "channel")))] pub struct Remote < Fut : Future > { tx : Option < Sender < SendMsg < Fut >>>, keep_running : Arc < AtomicBool >, # [pin] future : CatchUnwind < AssertUnwindSafe < Fut >>, } }
    };
}

macro_74!();