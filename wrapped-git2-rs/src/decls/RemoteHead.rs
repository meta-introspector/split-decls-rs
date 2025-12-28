macro_rules! RemoteHead {
    () => {
        # [doc = " Description of a reference advertised by a remote server, given out on calls"] # [doc = " to `list`."] pub struct RemoteHead < 'remote > { raw : * const raw :: git_remote_head , _marker : marker :: PhantomData < & 'remote str > , }
    };
}

RemoteHead!()